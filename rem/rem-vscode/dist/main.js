"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/main.ts
var main_exports = {};
__export(main_exports, {
  activate: () => activate,
  deactivate: () => deactivate
});
module.exports = __toCommonJS(main_exports);
var vscode2 = __toESM(require("vscode"));

// src/client.ts
var import_node_child_process = require("node:child_process");
var RemDaemonClient = class {
  constructor(binPath, output) {
    this.binPath = binPath;
    this.output = output;
  }
  proc = null;
  nextId = 1;
  pending = /* @__PURE__ */ new Map();
  buf = "";
  ensureRunning() {
    if (this.proc) return;
    this.proc = (0, import_node_child_process.spawn)(this.binPath, [], { stdio: ["pipe", "pipe", "pipe"] });
    this.proc.stderr.on("data", (d) => this.output.append(`[rem-extract] ${d.toString()}`));
    this.proc.on("exit", (code, signal) => {
      this.output.appendLine(`[rem-extract] exited (code=${code}, signal=${signal})`);
      for (const [, p] of this.pending) p.reject(new Error("daemon exited"));
      this.pending.clear();
      this.proc = null;
    });
    this.proc.stdout.on("data", (chunk) => this.onStdout(chunk.toString("utf8")));
  }
  onStdout(data) {
    this.buf += data;
    let idx;
    while ((idx = this.buf.indexOf("\n")) >= 0) {
      const line = this.buf.slice(0, idx).trim();
      this.buf = this.buf.slice(idx + 1);
      if (!line) continue;
      try {
        const resp = JSON.parse(line);
        const id = resp.id ?? null;
        if (id && this.pending.has(id)) {
          const p = this.pending.get(id);
          this.pending.delete(id);
          resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || "unknown error"));
        } else {
          const firstKey = this.pending.keys().next().value;
          if (firstKey) {
            const p = this.pending.get(firstKey);
            this.pending.delete(firstKey);
            resp.ok ? p.resolve(resp.data) : p.reject(new Error(resp.error || "unknown error"));
          }
        }
      } catch (e) {
        this.output.appendLine(`[rem-extract] bad JSON line: ${line}`);
      }
    }
  }
  async send(op, payload) {
    this.ensureRunning();
    if (!this.proc) throw new Error("daemon not running");
    const id = this.nextId++;
    const req = JSON.stringify({ id, op, ...payload }) + "\n";
    this.proc.stdin.write(req);
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
    });
  }
};

// src/check/checkEnv.ts
var import_child_process = require("child_process");
var os = __toESM(require("os"));
function commandExists(cmd) {
  try {
    if (os.platform() === "win32") {
      (0, import_child_process.execSync)(`where ${cmd}`, { stdio: "ignore" });
    } else {
      (0, import_child_process.execSync)(`which ${cmd}`, { stdio: "ignore" });
    }
    return true;
  } catch {
    return false;
  }
}
async function checkAll() {
  const tools = ["rustup", "opam", "coqc", "aeneas", "charon", "rem-cli"];
  const missing = tools.filter((t) => !commandExists(t));
  if (missing.length > 0) {
    throw new Error(missing.join(", "));
  }
}

// src/interface.ts
function isExtractData(d) {
  const x = d;
  return !!x && typeof x.output === "string" && typeof x.callsite === "string";
}
function isApplyData(d) {
  const x = d;
  return !!x && (x.status === "applied" || x.status === "no-op" || x.status === "no-change");
}
var buildInit = (manifest_path) => ({ manifest_path });
var buildChange = (path, text) => ({ path, text });
var buildExtract = (file, new_fn_name, start, end) => ({ file, new_fn_name, start, end });
var DEFAULT_DAEMON_SETTING_KEY = "remvscode.daemonPath";

// src/extract.ts
var vscode = __toESM(require("vscode"));
async function reinitDaemonForPath(client, manifestOrFile) {
  const payload = buildInit(manifestOrFile);
  const resp = await client.send("init", payload);
  const data = resp.ok !== void 0 ? resp.data : resp;
  if (!data) {
    throw new Error("Init returned no data");
  }
  return data;
}
async function sendChange(client, filePath, text) {
  const payload = buildChange(filePath, text);
  const data = await client.send("change", payload);
  if (!isApplyData(data)) {
    throw new Error(`Unexpected change response shape: ${JSON.stringify(data)}`);
  }
  return data;
}
async function runExtract(client, filePath, newFnName, start, end, currentText) {
  if (currentText !== void 0) {
    await sendChange(client, filePath, currentText);
  }
  const payload = buildExtract(filePath, newFnName, start, end);
  const data = await client.send("extract", payload);
  if (!isExtractData(data)) {
    throw new Error(`Unexpected extract response shape: ${JSON.stringify(data)}`);
  }
  return data;
}
async function extractFromActiveEditor(client, options) {
  const editor = vscode.window.activeTextEditor;
  if (!editor) {
    vscode.window.showErrorMessage("No active editor");
    return;
  }
  const doc = editor.document;
  const sel = editor.selection;
  const start = doc.offsetAt(sel.start);
  const end = doc.offsetAt(sel.end);
  const file = doc.uri.fsPath;
  const name = await vscode.window.showInputBox({
    prompt: options?.prompt ?? "Enter the new function name",
    placeHolder: options?.defaultName ?? "extracted_function"
  });
  if (!name) {
    return;
  }
  try {
    await reinitDaemonForPath(client, file);
    const data = await runExtract(client, file, name, start, end, doc.getText());
    if (options?.preview !== false) {
      const preview = await vscode.workspace.openTextDocument({ language: "rust", content: data.output });
      vscode.window.showTextDocument(preview, { preview: true });
    } else {
      vscode.window.showInformationMessage("Extraction completed.");
    }
  } catch (e) {
    vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
  }
}

// src/main.ts
var INSTALL_BASE = "https://github.com/RuleBrittonica/rem-vscode/scripts";
async function activate(context) {
  try {
    await checkAll();
  } catch (err) {
    const missingMsg = err instanceof Error ? err.message : String(err);
    const platformKey = process.platform === "win32" ? "windows" : process.platform === "darwin" ? "mac" : "linux";
    const url = `${INSTALL_BASE}/${platformKey}/`;
    const choice = await vscode2.window.showErrorMessage(
      `Missing dependencies: ${missingMsg}`,
      "Open install guide"
    );
    if (choice === "Open install guide") {
      vscode2.env.openExternal(vscode2.Uri.parse(url));
    }
    return;
  }
  const output = vscode2.window.createOutputChannel("REM");
  context.subscriptions.push(output);
  const config = vscode2.workspace.getConfiguration("remvscode");
  const binDir = config.get("aeneasBinariesPath") || defaultBinPath();
  const daemonPath = config.get(DEFAULT_DAEMON_SETTING_KEY) || "rem-extract";
  const charonPath = `${binDir}/charon`;
  const aeneasPath = `${binDir}/aeneas`;
  const client = new RemDaemonClient(daemonPath, output);
  vscode2.window.showInformationMessage(
    `All dependencies found. 
 - Binaries at ${binDir} 
 - Using REM daemon: ${daemonPath}`
  );
  const cmdReinit = vscode2.commands.registerCommand("remvscode.reinit", async () => {
    const doc = vscode2.window.activeTextEditor?.document;
    if (!doc) {
      return vscode2.window.showErrorMessage("No active editor");
    }
    try {
      await reinitDaemonForPath(client, doc.uri.fsPath);
      vscode2.window.showInformationMessage("REM database reinitialized");
    } catch (e) {
      vscode2.window.showErrorMessage(`Init failed: ${e.message || e}`);
    }
  });
  const cmdExtract = vscode2.commands.registerCommand("remvscode.extract", async () => {
    await extractFromActiveEditor(client, { preview: true });
  });
  const cmdRepair = vscode2.commands.registerCommand("remvscode.repair", async () => {
    vscode2.window.showInformationMessage("Repair command not implemented yet.");
  });
  const cmdExtractRepair = vscode2.commands.registerCommand("remvscode.extractRepair", async () => {
    vscode2.window.showInformationMessage("Extract & Repair command not implemented yet.");
  });
  const cmdExtractVerify = vscode2.commands.registerCommand("remvscode.extractVerify", async () => {
    vscode2.window.showInformationMessage("Extract & Verify command not implemented yet.");
  });
  const cmdExtractRepairVerify = vscode2.commands.registerCommand("remvscode.extractRepairVerify", async () => {
    vscode2.window.showInformationMessage("Extract, Repair & Verify command not implemented yet.");
  });
  context.subscriptions.push(
    cmdReinit,
    cmdExtract,
    cmdRepair,
    cmdExtractRepair,
    cmdExtractVerify,
    cmdExtractRepairVerify
  );
}
function deactivate() {
}
function defaultBinPath() {
  if (process.platform === "win32") {
    return `${process.env.USERPROFILE}\\bin`;
  } else {
    return `${process.env.HOME}/.local/bin`;
  }
}
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  activate,
  deactivate
});
//# sourceMappingURL=main.js.map
