import { ChildProcessWithoutNullStreams, spawn } from 'node:child_process';
import * as vscode from 'vscode';

type Json = Record<string, any>;

export class RemDaemonClient {
  private proc: ChildProcessWithoutNullStreams | null = null;
  private nextId = 1;
  private pending = new Map<number, { resolve: (v: any) => void; reject: (e: any) => void }>();
  private buf = '';

  constructor(private readonly binPath: string, private readonly output: vscode.OutputChannel) {}

  ensureRunning() {
    if (this.proc) return;
    this.proc = spawn(this.binPath, [], { stdio: ['pipe', 'pipe', 'pipe'] });
    this.proc.stderr.on('data', d => this.output.append(`[rem-extract] ${d.toString()}`));
    this.proc.on('exit', (code, signal) => {
      this.output.appendLine(`[rem-extract] exited (code=${code}, signal=${signal})`);
      // Reject all in-flight
      for (const [, p] of this.pending) p.reject(new Error('daemon exited'));
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
      if (!line) continue;
      try {
        const resp = JSON.parse(line) as { ok: boolean; data?: any; error?: string; id?: number };
        const id = (resp as any).id ?? null;
        if (id && this.pending.has(id)) {
          const p = this.pending.get(id)!;
          this.pending.delete(id);
          resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || 'unknown error'));
        } else {
          // Fallback: no ids => resolve the oldest pending (simple mode)
          const firstKey = this.pending.keys().next().value;
          if (firstKey) {
            const p = this.pending.get(firstKey)!;
            this.pending.delete(firstKey);
            resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || 'unknown error'));
          }
        }
      } catch (e) {
        this.output.appendLine(`[rem-extract] bad JSON line: ${line}`);
      }
    }
  }

  async send(op: string, payload: Json): Promise<any> {
    this.ensureRunning();
    if (!this.proc) throw new Error('daemon not running');
    const id = this.nextId++;
    const req = JSON.stringify({ id, op, ...payload }) + '\n';
    this.proc.stdin.write(req);
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
    });
  }
}
