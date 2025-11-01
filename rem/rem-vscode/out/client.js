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
        if (this.proc)
            return;
        this.proc = (0, node_child_process_1.spawn)(this.binPath, [], { stdio: ['pipe', 'pipe', 'pipe'] });
        this.proc.stderr.on('data', d => this.output.append(`[rem-extract] ${d.toString()}`));
        this.proc.on('exit', (code, signal) => {
            this.output.appendLine(`[rem-extract] exited (code=${code}, signal=${signal})`);
            // Reject all in-flight
            for (const [, p] of this.pending)
                p.reject(new Error('daemon exited'));
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
            if (!line)
                continue;
            try {
                const resp = JSON.parse(line);
                const id = resp.id ?? null;
                if (id && this.pending.has(id)) {
                    const p = this.pending.get(id);
                    this.pending.delete(id);
                    resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || 'unknown error'));
                }
                else {
                    // Fallback: no ids => resolve the oldest pending (simple mode)
                    const firstKey = this.pending.keys().next().value;
                    if (firstKey) {
                        const p = this.pending.get(firstKey);
                        this.pending.delete(firstKey);
                        resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || 'unknown error'));
                    }
                }
            }
            catch (e) {
                this.output.appendLine(`[rem-extract] bad JSON line: ${line}`);
            }
        }
    }
    async send(op, payload) {
        this.ensureRunning();
        if (!this.proc)
            throw new Error('daemon not running');
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