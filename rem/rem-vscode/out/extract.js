"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.reinitDaemonForPath = reinitDaemonForPath;
exports.sendChange = sendChange;
exports.runExtract = runExtract;
exports.extractFromActiveEditor = extractFromActiveEditor;
// src/extract.ts
const vscode = __importStar(require("vscode"));
const interface_1 = require("./interface");
/** Re-initialize the daemon DB using a path (Cargo.toml or .rs). */
async function reinitDaemonForPath(client, manifestOrFile) {
    const payload = (0, interface_1.buildInit)(manifestOrFile);
    const resp = (await client.send('init', payload));
    // client returns resp.data (because server wraps); handle both shapes just in case
    const data = resp.ok !== undefined ? resp.data : resp;
    if (!data) {
        throw new Error('Init returned no data');
    }
    return data;
}
/** Push the active buffer contents to daemon (or let server read from disk if text omitted). */
async function sendChange(client, filePath, text) {
    const payload = (0, interface_1.buildChange)(filePath, text);
    const data = await client.send('change', payload);
    if (!(0, interface_1.isApplyData)(data)) {
        throw new Error(`Unexpected change response shape: ${JSON.stringify(data)}`);
    }
    return data;
}
/** Perform extraction; returns the modified source and callsite snippet. */
async function runExtract(client, filePath, newFnName, start, end, 
/** Optional current buffer text (unsaved). If given, we send a change first. */
currentText) {
    if (currentText !== undefined) {
        await sendChange(client, filePath, currentText);
    }
    const payload = (0, interface_1.buildExtract)(filePath, newFnName, start, end);
    const data = await client.send('extract', payload);
    if (!(0, interface_1.isExtractData)(data)) {
        throw new Error(`Unexpected extract response shape: ${JSON.stringify(data)}`);
    }
    return data;
}
/** Convenience: extract from current editor selection, prompting for a name. */
async function extractFromActiveEditor(client, options) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }
    const doc = editor.document;
    const sel = editor.selection;
    const start = doc.offsetAt(sel.start);
    const end = doc.offsetAt(sel.end);
    const file = doc.uri.fsPath;
    const name = await vscode.window.showInputBox({
        prompt: options?.prompt ?? 'Enter the new function name',
        placeHolder: options?.defaultName ?? 'extracted_function',
    });
    if (!name) {
        return;
    }
    try {
        // init on first use (idempotent on server side)
        await reinitDaemonForPath(client, file);
        // push current buffer (even if unsaved)
        const data = await runExtract(client, file, name, start, end, doc.getText());
        // show result (preview tab)
        if (options?.preview !== false) {
            const preview = await vscode.workspace.openTextDocument({ language: 'rust', content: data.output });
            vscode.window.showTextDocument(preview, { preview: true });
        }
        else {
            vscode.window.showInformationMessage('Extraction completed.');
        }
    }
    catch (e) {
        vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
    }
}
//# sourceMappingURL=extract.js.map