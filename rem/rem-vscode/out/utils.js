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
exports.toLocalFsPath = toLocalFsPath;
exports.updateFileContents = updateFileContents;
const vscode = __importStar(require("vscode"));
/** Convert a possibly-URI string (e.g., "file:///…") to a local filesystem path. */
function toLocalFsPath(input) {
    // Heuristics: if it looks like a URI (scheme://), parse via VSCode; else return as-is.
    // vscode.Uri.parse handles file://, encoded chars, and platform-specific fsPath.
    if (/^[a-zA-Z][a-zA-Z0-9+\-.]*:\/\//.test(input) || input.startsWith('file:')) {
        try {
            return vscode.Uri.parse(input).fsPath;
        }
        catch {
            // If parsing fails, fall back to original string.
            return input;
        }
    }
    return input;
}
/**
 * Updates a file's contents on disk (and in any open editor) safely.
 *
 * @param filePath Full filesystem path to the file (e.g., `uri.fsPath`)
 * @param newContent The new text to replace the entire document
 * @returns true if successful, false if failed
 */
async function updateFileContents(filePath, newContent) {
    const uri = vscode.Uri.file(filePath);
    try {
        // Open the file as a text document
        const doc = await vscode.workspace.openTextDocument(uri);
        // Show it in an editor — required to apply edits
        // Preserve original view! Only reveal if not already visible.
        const alreadyVisible = vscode.window.visibleTextEditors.some(e => e.document.uri.fsPath === filePath);
        if (!alreadyVisible) {
            await vscode.window.showTextDocument(doc, { preview: false });
        }
        // Replace the entire document content
        const fullRange = new vscode.Range(doc.positionAt(0), doc.positionAt(doc.getText().length));
        const edit = new vscode.WorkspaceEdit();
        edit.replace(uri, fullRange, newContent);
        const success = await vscode.workspace.applyEdit(edit);
        if (success) {
            await doc.save(); // persist to disk
            return true;
        }
        vscode.window.showErrorMessage("Failed to apply edit to document.");
        return false;
    }
    catch (err) {
        vscode.window.showErrorMessage(`Failed to update file: ${filePath}\n${err.message || err}`);
        return false;
    }
}
//# sourceMappingURL=utils.js.map