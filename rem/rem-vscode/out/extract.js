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
const vscode = __importStar(require("vscode"));
const interface_1 = require("./interface");
const utils_1 = require("./utils");
/** Re-initialize the daemon DB using a path (Cargo.toml or .rs). */
async function reinitDaemonForPath(client, manifestOrFile) {
    const payload = (0, interface_1.buildInit)(manifestOrFile);
    const resp = await client.send('init', payload);
    if (!resp.ok) {
        throw new Error(resp.error);
    }
    return resp.data;
}
/** Push the active buffer contents to daemon (or let server read from disk if text omitted). */
async function sendChange(client, filePath, text) {
    const payload = (0, interface_1.buildChange)(filePath, text);
    const resp = await client.send('change', payload);
    if (!resp.ok) {
        // Surface but do not throw, so the caller can decide to continue.
        vscode.window.showErrorMessage(`Change failed: ${resp.error}`);
        return { status: 'no-op' };
    }
    return resp.data;
}
/** Perform extraction; returns the modified source and callsite snippet. */
async function runExtract(client, filePath, newFnName, start, end, 
/** Optional current buffer text (unsaved). If given, we send a change first. */
currentText) {
    if (currentText !== undefined) {
        await sendChange(client, filePath, currentText);
    }
    // Filepaths returned by VSCode might be URLs - if so we need to convert them
    // to local paths (applicable to the OS)
    const localPath = (0, utils_1.toLocalFsPath)(filePath);
    const payload = (0, interface_1.buildExtract)(localPath, newFnName, start, end);
    const resp = await client.send('extract', payload);
    if (!(0, interface_1.isOk)(resp)) {
        vscode.window.showErrorMessage(`Extract failed: ${resp.error}`);
        return { output: '', callsite: '' };
    }
    return resp.data;
}
/** Convenience: extract from current editor selection, prompting for a name. */
async function extractFromActiveEditor(client, options) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return null;
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
        return null;
    }
    try {
        // push current buffer (even if unsaved)
        const data = await runExtract(client, file, name, start, end, doc.getText());
        if (!data) {
            vscode.window.showErrorMessage('Extract failed: received no data');
            return null;
        }
        // Return ExtractData for caller to handle (e.g. updating source file)
        return data;
    }
    catch (e) {
        vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
        return null;
    }
}
//# sourceMappingURL=extract.js.map