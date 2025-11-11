import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import { VerifyData, buildVerify, isOk } from './interface';
import { toLocalFsPath } from './utils';

export async function runVerify(
  client: RemDaemonClient,
  file_path: string,
  original_text: string,
  refactored_text: string,
  fn_name: string, // callsite (caller fn name)
  charon_path: string,
  aeneas_path: string,
): Promise<VerifyData> {
  // Filepaths returned by VSCode might be URLs - if so we need to convert them
  // to local paths (applicable to the OS)
  const localPath = toLocalFsPath(file_path);

  const payload = buildVerify(localPath, original_text, refactored_text, fn_name, charon_path, aeneas_path);
  const resp = await client.send<VerifyData>('verify', payload);

  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Verify failed: ${resp.error || 'unknown error'}`);
    throw new Error(`Verify failed: ${resp.error || 'unknown error'}`);
  }

  return resp.data;
}
