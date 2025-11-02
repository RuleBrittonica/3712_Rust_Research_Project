"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RemDaemonClient = void 0;
const node_child_process_1 = require("node:child_process");
class RemDaemonClient {
    binPath;
    output;
    proc = null;
    nextId = 1;
    pending = new Map();
    buf = '';
    constructor(binPath, output) {
        this.binPath = binPath;
        this.output = output;
    }
    ensureRunning() {
        if (this.proc) {
            return;
        }
        this.proc = (0, node_child_process_1.spawn)(this.binPath, [], { stdio: ['pipe', 'pipe', 'pipe'] });
        this.proc.stderr.on('data', d => this.output.append(`[rem-server] ${d.toString()}`));
        this.proc.on('exit', (code, signal) => {
            this.output.appendLine(`[rem-server] exited (code=${code}, signal=${signal})`);
            // Reject all in-flight
            for (const [, p] of this.pending) {
                p.reject(new Error('daemon exited'));
            }
            this.pending.clear();
            this.proc = null;
        });
        this.proc.stdout.on('data', chunk => this.onStdout(chunk.toString('utf8')));
    }
    onStdout(data) {
        this.buf += data;
        let idx;
        while ((idx = this.buf.indexOf('\n')) >= 0) {
            const line = this.buf.slice(0, idx).trim();
            this.buf = this.buf.slice(idx + 1);
            if (!line) {
                continue;
            }
            try {
                // Expect the daemon to emit one JSON object per line.
                // Shape: { ok: true, data: ... , id?: number } | { ok: false, error: "...", id?: number }
                const resp = JSON.parse(line);
                const id = resp.id ?? null;
                const resolvePending = (r) => {
                    // Resolve (do not reject) so callers can discriminate by r.ok
                    const firstKey = this.pending.keys().next().value;
                    if (!firstKey) {
                        return;
                    }
                    const p = this.pending.get(firstKey);
                    this.pending.delete(firstKey);
                    p.resolve(r);
                };
                if (id && this.pending.has(id)) {
                    const p = this.pending.get(id);
                    this.pending.delete(id);
                    p.resolve(resp);
                }
                else {
                    // Fallback: resolve the oldest pending when no id is present
                    resolvePending(resp);
                }
            }
            catch (e) {
                this.output.appendLine(`[rem-server] bad JSON line: ${line}`);
            }
        }
    }
    // Make send<T> generic and return the full union to callers
    async send(op, payload) {
        this.ensureRunning();
        if (!this.proc) {
            throw new Error('daemon not running');
        }
        const id = this.nextId++;
        const req = JSON.stringify({ id, op, ...payload }) + '\n';
        this.proc.stdin.write(req);
        return new Promise((resolve, reject) => {
            this.pending.set(id, { resolve, reject });
        });
    }
}
exports.RemDaemonClient = RemDaemonClient;
//# sourceMappingURL=client.js.map