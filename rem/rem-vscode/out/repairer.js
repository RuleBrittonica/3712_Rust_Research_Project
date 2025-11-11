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
exports.runRepair = runRepair;
const vscode = __importStar(require("vscode"));
const interface_1 = require("./interface");
const utils_1 = require("./utils");
async function runRepair(client, filePath, newFnName) {
    // Filepaths returned by VSCode might be URLs - if so we need to convert them
    // to local paths (applicable to the OS)
    const localPath = (0, utils_1.toLocalFsPath)(filePath);
    const payload = (0, interface_1.buildRepair)(localPath, newFnName);
    const resp = await client.send('repair', payload);
    if (!(0, interface_1.isOk)(resp)) {
        vscode.window.showErrorMessage(`Repair failed: ${resp.error || 'unknown error'}`);
        throw new Error(resp.error || 'unknown error');
    }
    return resp.data;
}
//# sourceMappingURL=repairer.js.map