"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEFAULT_DAEMON_SETTING_KEY = exports.buildRepair = exports.buildExtract = exports.buildDelete = exports.buildChange = exports.buildCreate = exports.buildInit = void 0;
exports.isOk = isOk;
function isOk(r) {
    return r.ok === true;
}
/**  Small helpers to build request payloads */
const buildInit = (manifest_path) => ({ manifest_path });
exports.buildInit = buildInit;
const buildCreate = (path, text) => ({ path, text });
exports.buildCreate = buildCreate;
const buildChange = (path, text) => ({ path, text });
exports.buildChange = buildChange;
const buildDelete = (path) => ({ path });
exports.buildDelete = buildDelete;
const buildExtract = (file, new_fn_name, start, end) => ({ file, new_fn_name, start, end });
exports.buildExtract = buildExtract;
const buildRepair = (file, new_fn_name) => ({ file, new_fn_name });
exports.buildRepair = buildRepair;
exports.DEFAULT_DAEMON_SETTING_KEY = 'remvscode.daemonPath'; // points at rem-extract for now
//# sourceMappingURL=interface.js.map