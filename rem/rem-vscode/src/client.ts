import { ChildProcessWithoutNullStreams, spawn } from 'node:child_process';
import * as vscode from 'vscode';
import {
  InitPayload, CreatePayload, ChangePayload, ExtractPayload, DeletePayload,
  InitData, ApplyData, ExtractData,
  JsonResp
} from './interface';

type Json = Record<string, any>;

export class RemDaemonClient {
  private proc: ChildProcessWithoutNullStreams | null = null;
  private nextId = 1;
  private pending = new Map<number, { resolve: (v: any) => void; reject: (e: any) => void }>();
  private buf = '';

  constructor(private readonly binPath: string, private readonly output: vscode.OutputChannel) {}

  ensureRunning() {
    if (this.proc) {return;}
    this.proc = spawn(this.binPath, [], { stdio: ['pipe', 'pipe', 'pipe'] });
    this.proc.stderr.on('data', d => this.output.append(`[rem-server] ${d.toString()}`));
    this.proc.on('exit', (code, signal) => {
      this.output.appendLine(`[rem-server] exited (code=${code}, signal=${signal})`);
      // Reject all in-flight
      for (const [, p] of this.pending) {p.reject(new Error('daemon exited'));}
      this.pending.clear();
      this.proc = null;
    });
    this.proc.stdout.on('data', chunk => this.onStdout(chunk.toString('utf8')));
  }

  private onStdout(data: string) {
    this.buf += data;
    let idx: number;
    while ((idx = this.buf.indexOf('\n')) >= 0) {
      const line = this.buf.slice(0, idx).trim();
      this.buf = this.buf.slice(idx + 1);
      if (!line) {continue;}
      this.output.appendLine(`[recv] ${line}`);
      try {
        // Expect the daemon to emit one JSON object per line.
        // Shape: { ok: true, data: ... , id?: number } | { ok: false, error: "...", id?: number }
        const resp = JSON.parse(line) as (JsonResp<any> & { id?: number });
        const id = (resp as any).id ?? null;

        const resolvePending = (r: JsonResp<any>) => {
          // Resolve (do not reject) so callers can discriminate by r.ok
          const firstKey = this.pending.keys().next().value;
          if (!firstKey) { return; }
          const p = this.pending.get(firstKey)!;
          this.pending.delete(firstKey);
          p.resolve(r);
        };

        if (id && this.pending.has(id)) {
          const p = this.pending.get(id)!;
          this.pending.delete(id);
          p.resolve(resp);
        } else {
          // Fallback: resolve the oldest pending when no id is present
          resolvePending(resp);
        }
      } catch (e) {
        this.output.appendLine(`[rem-server] bad JSON line: ${line}`);
      }
    }
  }


  // Make send<T> generic and return the full union to callers
  async send<T = unknown>(op: string, payload: Json): Promise<JsonResp<T>> {
    this.ensureRunning();
    if (!this.proc) {throw new Error('daemon not running');}
    const id = this.nextId++;
    const req = JSON.stringify({ id, op, ...payload }) + '\n';
    this.output.appendLine(`[send] ${req.trim()}`);
    this.proc.stdin.write(req);
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
    });
  }
}