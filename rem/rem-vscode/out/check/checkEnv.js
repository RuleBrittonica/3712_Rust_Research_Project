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
exports.checkAll = checkAll;
const child_process_1 = require("child_process");
const os = __importStar(require("os"));
function commandExists(cmd) {
    try {
        if (os.platform() === 'win32') {
            (0, child_process_1.execSync)(`where ${cmd}`, { stdio: 'ignore' });
        }
        else {
            (0, child_process_1.execSync)(`which ${cmd}`, { stdio: 'ignore' });
        }
        return true;
    }
    catch {
        return false;
    }
}
/**
 * Throws an Error whose .message is a comma-separated list of missing tools.
 */
async function checkAll() {
    const tools = ['rustup', 'opam', 'coqc', 'aeneas', 'charon', 'rem-cli'];
    const missing = tools.filter(t => !commandExists(t));
    if (missing.length > 0) {
        throw new Error(missing.join(', '));
    }
}
//# sourceMappingURL=checkEnv.js.map