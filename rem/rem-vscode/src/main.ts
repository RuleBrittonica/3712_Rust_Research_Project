import * as vscode from 'vscode';
import { RemDaemonClient } from './client';
import { checkAll } from './check/checkEnv';
import { DEFAULT_DAEMON_SETTING_KEY } from './interface';
import { extractFromActiveEditor, initDaemonForPath, reinitDaemonForPath, runExtractFile } from './extract';
import { runRepair } from './repairer';

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
  const daemonPath = config.get<string>(DEFAULT_DAEMON_SETTING_KEY) || '/home/matt/3712_Rust_Research_Project/rem/target/release/rem-server';
  const charonPath = `${binDir}/charon`;
  const aeneasPath = `${binDir}/aeneas`;


  vscode.window.showInformationMessage(
    `All dependencies found. \n - Binaries at ${binDir} \n - Using REM daemon: ${daemonPath}`
  );

  // 3) Start the daemon client and initialise the database
  const client = new RemDaemonClient(daemonPath, output);
  const doc = vscode.window.activeTextEditor?.document;
  if (!doc) {
    vscode.window.showErrorMessage('No active editor');
  }
  client.ensureRunning();
  // await initDaemonForPath(client, doc!.uri.fsPath);

  // Register normal user commands
  // 1) Extract and Apply Immediately
  const cmdExtract = vscode.commands.registerCommand('remvscode.extract', async () => {
    let name = await vscode.window.showInputBox({
      prompt: 'Enter the new function name',
      placeHolder: 'extracted_function',
    });

    // Simple timing metrics for results
    // const tStart = performance.now();

    if (!name) {
      name = "extracted_function";
    }

    const extract_data = await runExtractFile(client, name);

    if (extract_data) {
      const new_src = extract_data.output;
      const file_path = extract_data.file;

      try {
        await applyWorkspaceEdit(file_path, new_src);

        // const totalMs = performance.now() - tStart;
        // output.appendLine(`[Metrics] Extract: ${totalMs.toFixed(2)} ms`);

        vscode.window.showInformationMessage('Extract applied successfully.');
      } catch (e: any) {
        // const totalMs = performance.now() - tStart;
        // output.appendLine(`[Metrics] Extract failed after ${totalMs.toFixed(2)} ms`);

        vscode.window.showErrorMessage(`Failed to apply extract: ${e.message || e}`);
      }
    } else {
      // const totalMs = performance.now() - tStart;
      // output.appendLine(`[Metrics] Extract cancelled or failed: ${totalMs.toFixed(2)} ms`);

      vscode.window.showInformationMessage('Extract cancelled or failed.');
    }
  });

  // 2) Repair
  // const cmdRepair = vscode.commands.registerCommand('remvscode.repair', async () => {
  //   vscode.window.showInformationMessage('Repair command not implemented yet.');
  // });

  // 3) Extract and Repair
  const cmdExtractRepair = vscode.commands.registerCommand('remvscode.extractRepair', async () => {
    let name = await vscode.window.showInputBox({
      prompt: 'Enter the new function name',
      placeHolder: 'extracted_function',
    });

    if (!name) {
      name = "extracted_function";
    }

    const extract_data = await runExtractFile(client, name);

    if (!extract_data) {
      return vscode.window.showInformationMessage('Extract cancelled or failed.');
    }

    const new_src = extract_data.output;
    const file_path = extract_data.file;
    const callsite = extract_data.callsite;

    // Try to update the file and then call the repairer
    try {
      await applyWorkspaceEdit(file_path, new_src);
    } catch (e: any) {
      return vscode.window.showErrorMessage(`Failed to apply extract: ${e.message || e}`);
    }

    // Now call the repairer on the modified file
    const repair_data = await runRepair(client, file_path, callsite);

    if (!repair_data) {
      return vscode.window.showInformationMessage('Repair cancelled or failed.');
    }

    const id = repair_data.idx;
    const system = repair_data.system_name;
    const count = repair_data.repair_count;
    const changed = repair_data.changed_files;

    // Show some repair summary info
    return vscode.window.showInformationMessage(
      `Extract and Repair #${id} completed on system "${system}". Total repairs: ${count}. Changed files: ${changed}`
    );
  });

  // 4) Extract and Verify
  const cmdExtractVerify = vscode.commands.registerCommand('remvscode.extractVerify', async () => {
    vscode.window.showInformationMessage('Extract & Verify command not implemented yet.');
  });

  // 5) Extract, Repair, and Verify
  const cmdExtractRepairVerify = vscode.commands.registerCommand('remvscode.extractRepairVerify', async () => {
    vscode.window.showInformationMessage('Extract, Repair & Verify command not implemented yet.');
  });

  // 6) Reinit command (Command Palette): reinitialize database for current file
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

  context.subscriptions.push(
    cmdReinit,
    cmdExtract,
    // cmdRepair,
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

  // make sure that we have content changes (otherwise we have probably errored
  // earlier up the line and don't want to do anything here)
  if (newContent === "") {
    throw new Error('New content is empty, aborting edit');
  }

  const edit = new vscode.WorkspaceEdit();
  edit.replace(uri, fullRange, newContent);
  const ok = await vscode.workspace.applyEdit(edit);
  if (!ok) {throw new Error('Failed to apply workspace edit');}

  // Immediately save the document
  await doc.save();
}