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
exports.setHasPreview = setHasPreview;
exports.getPreviewKey = getPreviewKey;
exports.invalidateIfDocChanged = invalidateIfDocChanged;
const vscode = __importStar(require("vscode"));
let previewKey;
function setHasPreview(key) {
    previewKey = key;
    void vscode.commands.executeCommand('setContext', 'remvscode.hasExtractPreview', !!key);
}
function getPreviewKey() { return previewKey; }
function invalidateIfDocChanged(e) {
    if (!previewKey) {
        return;
    }
    if (e.document.uri.toString() === previewKey.docUri && e.document.version !== previewKey.version) {
        setHasPreview(undefined);
    }
}
// export function invalidateIfSelectionChanged(e: vscode.TextEditorSelectionChangeEvent) {
//   if (!previewKey || e.textEditor.document.uri.toString() !== previewKey.docUri) {return;}
//   const sel = e.selections[0];
//   if (!sel || !sel.range.isEqual(previewKey.selection)) {
//     setHasPreview(undefined);
//   }
// }
//# sourceMappingURL=preview.js.map