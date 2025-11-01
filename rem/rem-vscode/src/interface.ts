/** All operations supported over stdio. */
export type Op =
  | 'init'
  | 'create'
  | 'change'
  | 'delete'
  | 'extract'
  | 'repair'
  | 'verify';

/** Generic JSON response envelope returned by the daemon. */
export interface JsonResp<T = unknown> {
  ok: boolean;
  data?: T;
  error?: string;
}

/** Requests: payloads  */

export interface InitPayload {
  /** Path to Cargo.toml or any .rs file; server resolves upward to the manifest. */
  manifest_path: string;
}

export interface CreatePayload {
  path: string;
  /** Optional: send unsaved buffer contents. If omitted, server reads from disk. */
  text?: string;
}

export interface ChangePayload {
  path: string;
  /** Optional: send unsaved buffer contents. If omitted, server reads from disk. */
  text?: string;
}

export interface DeletePayload {
  path: string;
}

export interface ExtractPayload {
  file: string;
  new_fn_name: string;
  start: number; // byte offset (inclusive)
  end: number;   // byte offset (exclusive)
}

/** Responses: data shapes */

export interface InitData {
  status: 'initialized';
}

export type ApplyStatus = 'applied' | 'no-op' | 'no-change';
export interface ApplyData {
  status: ApplyStatus;
  path?: string;
}

export interface ExtractData {
  /** The full modified file text to preview/replace. */
  output: string;
  /** The callsite (or caller method snippet) if you surface it separately. */
  callsite: string;
}

/** Future placeholders */
export interface RepairPayload {
  // e.g., file/module identifiers, options, etc.
}
export interface VerifyPayload {
  // e.g., proof target config, options, etc.
}
export interface RepairData {
  // results/artifacts/log summaries
}
export interface VerifyData {
  // results/artifacts/log summaries
}

/** ===== Type guards (handy for narrowing) ===== */
export function isExtractData(d: unknown): d is ExtractData {
  const x = d as any;
  return !!x && typeof x.output === 'string' && typeof x.callsite === 'string';
}

export function isApplyData(d: unknown): d is ApplyData {
  const x = d as any;
  return !!x && (x.status === 'applied' || x.status === 'no-op' || x.status === 'no-change');
}

/** ===== Small helpers to build request payloads (optional sugar) ===== */
export const buildInit = (manifest_path: string): InitPayload => ({ manifest_path });
export const buildCreate = (path: string, text?: string): CreatePayload => ({ path, text });
export const buildChange = (path: string, text?: string): ChangePayload => ({ path, text });
export const buildDelete = (path: string): DeletePayload => ({ path });
export const buildExtract = (file: string, new_fn_name: string, start: number, end: number): ExtractPayload =>
  ({ file, new_fn_name, start, end });

export const DEFAULT_DAEMON_SETTING_KEY = 'remvscode.daemonPath'; // points at rem-extract for now
