import * as vscode from 'vscode';
import { exec, execSync } from 'node:child_process';
import * as os from 'os';
import { RemDaemonClient } from './client';
import { checkAll } from './check/checkEnv';
import { DEFAULT_DAEMON_SETTING_KEY } from './interface';
import { extractFromActiveEditor, reinitDaemonForPath } from './extract';

const INSTALL_BASE = 'https://github.com/RuleBrittonica/rem-vscode/scripts'

export async function activate(context: vscode.ExtensionContext) {
  // Only run on a rust file!
  // if (!(vscode.window.activeTextEditor?.document.languageId === 'rust')) {
  //   return;
  // }

  // 1) Check dependencies
  try {
    await checkAll();
  } catch (err: unknown) {
    const missingMsg = err instanceof Error ? err.message : String(err);
    const platformKey =
    process.platform === 'win32' ? 'windows'
      : process.platform === 'darwin' ? 'mac'
      : 'linux';

    const url = `${INSTALL_BASE}/${platformKey}/`;
    const choice = await vscode.window.showErrorMessage(
      `Missing dependencies: ${missingMsg}`,
      'Open install guide'
    );
    if (choice === 'Open install guide') {
      vscode.env.openExternal(vscode.Uri.parse(url));
    }

    return;
  }

  // 2) Inform user where the binaries live
  // 2) Paths / output channel
  const output = vscode.window.createOutputChannel('REM');
  context.subscriptions.push(output);

  const config = vscode.workspace.getConfiguration('remvscode');
  const binDir = config.get<string>('aeneasBinariesPath') || defaultBinPath();
  const daemonPath = config.get<string>(DEFAULT_DAEMON_SETTING_KEY) || 'rem-extract'; // <- switch to 'rem-server' later
  const charonPath = `${binDir}/charon`;
  const aeneasPath = `${binDir}/aeneas`;

  const client = new RemDaemonClient(daemonPath, output);

  vscode.window.showInformationMessage(
    `All dependencies found. \n - Binaries at ${binDir} \n - Using REM daemon: ${daemonPath}`
  );

  // 3) Reinit command (Command Palette): reinitialize database for current file
  // Command: Reinitialize DB for current file
  const cmdReinit = vscode.commands.registerCommand('remvscode.reinit', async () => {
    const doc = vscode.window.activeTextEditor?.document;
    if (!doc) {return vscode.window.showErrorMessage('No active editor');}
    try {
      await reinitDaemonForPath(client, doc.uri.fsPath);
      vscode.window.showInformationMessage('REM database reinitialized');
    } catch (e: any) {
      vscode.window.showErrorMessage(`Init failed: ${e.message || e}`);
    }
  });

  // Register normal user commands
  // 1) Extract and Apply Immediately
  const cmdExtract = vscode.commands.registerCommand('remvscode.extract', async () => {
    await extractFromActiveEditor(client, { preview: true });
  });

  // 2) Repair
  const cmdRepair = vscode.commands.registerCommand('remvscode.repair', async () => {
    vscode.window.showInformationMessage('Repair command not implemented yet.');
  });

  // 3) Extract and Repair
  const cmdExtractRepair = vscode.commands.registerCommand('remvscode.extractRepair', async () => {
    vscode.window.showInformationMessage('Extract & Repair command not implemented yet.');
  });

  // 4) Extract and Verify
  const cmdExtractVerify = vscode.commands.registerCommand('remvscode.extractVerify', async () => {
    vscode.window.showInformationMessage('Extract & Verify command not implemented yet.');
  });

  // 5) Extract, Repair, and Verify
  const cmdExtractRepairVerify = vscode.commands.registerCommand('remvscode.extractRepairVerify', async () => {
    vscode.window.showInformationMessage('Extract, Repair & Verify command not implemented yet.');
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

export function deactivate() {}

// fallback for default ~/.local/bin on *nix, ~/bin on Windows
function defaultBinPath(): string {
  if (process.platform === 'win32') {
    return `${process.env.USERPROFILE}\\bin`;
  } else {
    return `${process.env.HOME}/.local/bin`;
  }
}

// Apply a full-file replacement via WorkspaceEdit
async function applyWorkspaceEdit(absFilePath: string, newContent: string): Promise<void> {
  const uri = vscode.Uri.file(absFilePath);
  const doc = await vscode.workspace.openTextDocument(uri);
  const fullRange = new vscode.Range(
    doc.positionAt(0),
    doc.positionAt(doc.getText().length)
  );

  const edit = new vscode.WorkspaceEdit();
  edit.replace(uri, fullRange, newContent);
  const ok = await vscode.workspace.applyEdit(edit);
  if (!ok) {throw new Error('Failed to apply workspace edit');}

  // Immediately save the document
  await doc.save();
}