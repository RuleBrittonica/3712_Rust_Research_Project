import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import { ExtractData, buildExtract, isOk, ExtractPayload, } from './interface';

type PreviewKey = {
    docUri: string;
    version: number;
    selection: vscode.Range;
    when: number;
};

let previewKey: PreviewKey | undefined;

export function setHasPreview(key: PreviewKey | undefined) {
  previewKey = key;
  void vscode.commands.executeCommand('setContext', 'remvscode.hasExtractPreview', !!key);
}

export function getPreviewKey() { return previewKey; }

export function invalidateIfDocChanged(e: vscode.TextDocumentChangeEvent) {
  if (!previewKey) {return;}
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