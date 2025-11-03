import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import {
  InitData, ExtractData, ApplyData,
  buildInit, buildChange, buildExtract,
  isOk,
  ExtractPayload,
} from './interface';

import { toLocalFsPath } from './utils';
import { ClientRequest } from 'node:http';

/** Initialise the daemon DB using a path (Cargo.toml or .rs) */
export async function initDaemonForPath(client: RemDaemonClient, manifestOrFile: string): Promise<InitData> {
  const localPath = toLocalFsPath(manifestOrFile);
  const payload = buildInit(localPath);
  const resp = await client.send<InitData>('init', payload);
  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Init failed: ${resp.error}`);
    throw new Error(resp.error);
  }
  vscode.window.showInformationMessage(`Init succeeded for ${localPath}`);
  return resp.data;
}

/** Re-initialize the daemon DB using a path (Cargo.toml or .rs). */
export async function reinitDaemonForPath(client: RemDaemonClient, manifestOrFile: string): Promise<InitData> {
  const localPath = toLocalFsPath(manifestOrFile);
  const payload = buildInit(localPath);
  const resp = await client.send<InitData>('init', payload);
  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Reinit failed: ${resp.error}`);
    throw new Error(resp.error);
  }
  vscode.window.showInformationMessage(`Reinit succeeded for ${localPath}`);
  return resp.data;
}

/** Push the active buffer contents to daemon (or let server read from disk if text omitted). */
export async function sendChange(
  client: RemDaemonClient,
  filePath: string,
  text?: string
): Promise<ApplyData> {
  const payload = buildChange(filePath, text);
  const resp = await client.send<ApplyData>('change', payload);
  if (!isOk(resp)) {
    // Surface but do not throw, so the caller can decide to continue.
    vscode.window.showErrorMessage(`Change failed: ${resp.error}`);
    return { status: 'no-op' };
  }
  return resp.data;
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
  // Filepaths returned by VSCode might be URLs - if so we need to convert them
  // to local paths (applicable to the OS)
  const localPath = toLocalFsPath(filePath);

  // if (currentText !== undefined) {
  //   await sendChange(client, localPath, currentText);
  // }
  const payload = buildExtract(localPath, newFnName, start, end);
  const resp = await client.send<ExtractData>('extract', payload);

  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Extract failed: ${resp.error}`);
    return { output: '', callsite: '' };
  }

  return resp.data;
}

type ExtractDataWithFile = ExtractData & { file: string };

/** Convenience: extract from current editor selection, prompting for a name. */
export async function extractFromActiveEditor(
  client: RemDaemonClient,
  options?: { prompt?: string; defaultName?: string; preview?: boolean }
): Promise<ExtractDataWithFile | null> {
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

    // Return ExtractData along with the file path
    return { ...data, file };

  } catch (e: any) {
    vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
    return null;
  }
}

/** Faster no daemon pathway */
export async function runExtractFile(
  client: RemDaemonClient,
  name: string,
): Promise<ExtractDataWithFile | null> {
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

  try {
    const data = await extractFileServer(client, file, name, start, end);

    if (!data) {
      vscode.window.showErrorMessage('Extract failed: received no data');
      return null;
    }

    // Return ExtractData along with the file path
    return { ...data, file };
  } catch (e: any) {
    vscode.window.showErrorMessage(`Extract failed: ${e.message || e}`);
    return null;
  }
}

async function extractFileServer(
  client: RemDaemonClient,
  path: string,
  newFnName: string,
  start: number,
  end: number,
): Promise<ExtractData> {
  // Filepaths returned by VSCode might be URLs - if so we need to convert them
  // to local paths (applicable to the OS)
  const localPath = toLocalFsPath(path);

  // Directly read from file system
  const payload: ExtractPayload = buildExtract(localPath, newFnName, start, end);
  const resp = await client.send<ExtractData>('extract_file', payload);

  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Extract failed: ${resp.error}`);
    return { output: '', callsite: '' };
  }

  return resp.data;
}
