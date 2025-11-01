// src/extract.ts
import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import {
  JsonResp,
  InitData, ExtractData, ApplyData,
  buildInit, buildChange, buildExtract,
  isExtractData, isApplyData,
} from './interface';

/** Re-initialize the daemon DB using a path (Cargo.toml or .rs). */
export async function reinitDaemonForPath(client: RemDaemonClient, manifestOrFile: string): Promise<InitData> {
  const payload = buildInit(manifestOrFile);
  const resp = (await client.send('init', payload)) as JsonResp<InitData> | InitData;
  // client returns resp.data (because server wraps); handle both shapes just in case
  const data = (resp as any).ok !== undefined ? (resp as JsonResp<InitData>).data : (resp as InitData);
  if (!data) {throw new Error('Init returned no data');}
  return data;
}

/** Push the active buffer contents to daemon (or let server read from disk if text omitted). */
export async function sendChange(
  client: RemDaemonClient,
  filePath: string,
  text?: string
): Promise<ApplyData> {
  const payload = buildChange(filePath, text);
  const data = await client.send('change', payload);
  if (!isApplyData(data)) {throw new Error(`Unexpected change response shape: ${JSON.stringify(data)}`);}
  return data;
}

/** Perform extraction; returns the modified source and callsite snippet. */
export async function runExtract(
  client: RemDaemonClient,
  filePath: string,
  newFnName: string,
  start: number,
  end: number,
  /** Optional current buffer text (unsaved). If given, we send a change first. */
  currentText?: string,
): Promise<ExtractData> {
  if (currentText !== undefined) {
    await sendChange(client, filePath, currentText);
  }
  const payload = buildExtract(filePath, newFnName, start, end);
  const data = await client.send('extract', payload);
  if (!isExtractData(data)) {throw new Error(`Unexpected extract response shape: ${JSON.stringify(data)}`);}
  return data;
}

/** Convenience: extract from current editor selection, prompting for a name. */
export async function extractFromActiveEditor(
  client: RemDaemonClient,
  options?: { prompt?: string; defaultName?: string; preview?: boolean }
): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor) { vscode.window.showErrorMessage('No active editor'); return; }

  const doc = editor.document;
  const sel = editor.selection;
  const start = doc.offsetAt(sel.start);
  const end   = doc.offsetAt(sel.end);
  const file  = doc.uri.fsPath;

  const name = await vscode.window.showInputBox({
    prompt: options?.prompt ?? 'Enter the new function name',
    placeHolder: options?.defaultName ?? 'extracted_function',
  });
  if (!name) {return;}

  try {
    // init on first use (idempotent on server side)
    await reinitDaemonForPath(client, file);
    // push current buffer (even if unsaved)
    const data = await runExtract(client, file, name, start, end, doc.getText());

    // show result (preview tab)
    if (options?.preview !== false) {
      const preview = await vscode.workspace.openTextDocument({ language: 'rust', content: data.output });
      vscode.window.showTextDocument(preview, { preview: true });
    } else {
      vscode.window.showInformationMessage('Extraction completed.');
    }
  } catch (e: any) {
    vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
  }
}
