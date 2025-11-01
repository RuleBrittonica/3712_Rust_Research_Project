"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DEFAULT_DAEMON_SETTING_KEY = exports.buildExtract = exports.buildDelete = exports.buildChange = exports.buildCreate = exports.buildInit = void 0;
exports.isExtractData = isExtractData;
exports.isApplyData = isApplyData;
/** ===== Type guards (handy for narrowing) ===== */
function isExtractData(d) {
    const x = d;
    return !!x && typeof x.output === 'string' && typeof x.callsite === 'string';
}
function isApplyData(d) {
    const x = d;
    return !!x && (x.status === 'applied' || x.status === 'no-op' || x.status === 'no-change');
}
/** ===== Small helpers to build request payloads (optional sugar) ===== */
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
exports.DEFAULT_DAEMON_SETTING_KEY = 'remvscode.daemonPath'; // points at rem-extract for now
//# sourceMappingURL=interface.js.map