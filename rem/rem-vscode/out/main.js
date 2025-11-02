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
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const client_1 = require("./client");
const checkEnv_1 = require("./check/checkEnv");
const interface_1 = require("./interface");
const extract_1 = require("./extract");
const INSTALL_BASE = 'https://github.com/RuleBrittonica/rem-vscode/scripts';
async function activate(context) {
    // Only run on a rust file!
    // if (!(vscode.window.activeTextEditor?.document.languageId === 'rust')) {
    //   return;
    // }
    // 1) Check dependencies
    try {
        await (0, checkEnv_1.checkAll)();
    }
    catch (err) {
        const missingMsg = err instanceof Error ? err.message : String(err);
        const platformKey = process.platform === 'win32' ? 'windows'
            : process.platform === 'darwin' ? 'mac'
                : 'linux';
        const url = `${INSTALL_BASE}/${platformKey}/`;
        const choice = await vscode.window.showErrorMessage(`Missing dependencies: ${missingMsg}`, 'Open install guide');
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
    const binDir = config.get('aeneasBinariesPath') || defaultBinPath();
    const daemonPath = config.get(interface_1.DEFAULT_DAEMON_SETTING_KEY) || '/home/matt/3712_Rust_Research_Project/rem/target/release/rem-server';
    const charonPath = `${binDir}/charon`;
    const aeneasPath = `${binDir}/aeneas`;
    const client = new client_1.RemDaemonClient(daemonPath, output);
    vscode.window.showInformationMessage(`All dependencies found. \n - Binaries at ${binDir} \n - Using REM daemon: ${daemonPath}`);
    // 3) Reinit command (Command Palette): reinitialize database for current file
    // Command: Reinitialize DB for current file
    const cmdReinit = vscode.commands.registerCommand('remvscode.reinit', async () => {
        const doc = vscode.window.activeTextEditor?.document;
        if (!doc) {
            return vscode.window.showErrorMessage('No active editor');
        }
        try {
            await (0, extract_1.reinitDaemonForPath)(client, doc.uri.fsPath);
            vscode.window.showInformationMessage('REM database reinitialized');
        }
        catch (e) {
            vscode.window.showErrorMessage(`Init failed: ${e.message || e}`);
        }
    });
    // Register normal user commands
    // 1) Extract and Apply Immediately
    const cmdExtract = vscode.commands.registerCommand('remvscode.extract', async () => {
        await (0, extract_1.extractFromActiveEditor)(client, { preview: true });
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
    context.subscriptions.push(cmdReinit, cmdExtract, cmdRepair, cmdExtractRepair, cmdExtractVerify, cmdExtractRepairVerify);
}
function deactivate() { }
// fallback for default ~/.local/bin on *nix, ~/bin on Windows
function defaultBinPath() {
    if (process.platform === 'win32') {
        return `${process.env.USERPROFILE}\\bin`;
    }
    else {
        return `${process.env.HOME}/.local/bin`;
    }
}
// Apply a full-file replacement via WorkspaceEdit
async function applyWorkspaceEdit(absFilePath, newContent) {
    const uri = vscode.Uri.file(absFilePath);
    const doc = await vscode.workspace.openTextDocument(uri);
    const fullRange = new vscode.Range(doc.positionAt(0), doc.positionAt(doc.getText().length));
    const edit = new vscode.WorkspaceEdit();
    edit.replace(uri, fullRange, newContent);
    const ok = await vscode.workspace.applyEdit(edit);
    if (!ok) {
        throw new Error('Failed to apply workspace edit');
    }
    // Immediately save the document
    await doc.save();
}
//# sourceMappingURL=main.js.map