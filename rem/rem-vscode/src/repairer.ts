import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import { JsonResp, RepairPayload, RepairData, buildRepair, isOk } from './interface';
import { toLocalFsPath } from './utils';

export async function runRepair(
  client: RemDaemonClient,
  filePath: string,
  newFnName: string,
): Promise<RepairData> {
  // Filepaths returned by VSCode might be URLs - if so we need to convert them
  // to local paths (applicable to the OS)
  const localPath = toLocalFsPath(filePath);

  const payload = buildRepair(localPath, newFnName);
  const resp = await client.send<RepairData>('repair', payload);

  if (!isOk(resp)) {
    vscode.window.showErrorMessage(`Repair failed: ${resp.error || 'unknown error'}`);
    throw new Error(resp.error || 'unknown error');
  }

  return resp.data;
}
