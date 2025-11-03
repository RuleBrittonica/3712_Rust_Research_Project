#!/usr/bin/env python3
import argparse
import csv
import datetime as dt
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Tuple

INC_RE = re.compile(r"^inc:(.+?)->(.+?)\s+([0-9]*\.?[0-9]+)\s+s\s*$")

def normalize_key(frm: str, to: str) -> str:
    # derive a stable, “sensible” column name per metric edge
    def norm(s: str) -> str:
        return re.sub(r"[^a-z0-9]+", "_", s.strip().lower()).strip("_")
    return f"{norm(frm)}__to__{norm(to)}"

def parse_incremental_metrics(stdout: str) -> List[Tuple[str, str, float]]:
    """Return (from, to, seconds) for each inc: line."""
    out = []
    for line in stdout.splitlines():
        if not line.startswith("inc:"):
            continue
        m = INC_RE.match(line.strip())
        if m:
            out.append((m.group(1).strip(), m.group(2).strip(), float(m.group(3))))
    return out

def run_rem_extract(
    rem_extract: Path, file_path: Path, new_fn_name: str, start_index: int, end_index: int,
    enable_metrics: bool = True, verbose: bool = False, json_out: bool = False,
    extra_args: Optional[List[str]] = None, cwd: Optional[Path] = None
) -> Tuple[int, str, str]:
    cmd = [
        str(rem_extract), "extract", str(file_path),
        str(new_fn_name), str(start_index), str(end_index),
    ]
    if verbose:
        cmd.append("--verbose")
    if enable_metrics:
        cmd.append("--metrics")
    if json_out:
        cmd.append("--json")
    if extra_args:
        cmd.extend(extra_args)
    proc = subprocess.run(
        cmd, cwd=str(cwd) if cwd else None,
        capture_output=True, text=True, encoding="utf-8", errors="replace",
    )
    return proc.returncode, proc.stdout, proc.stderr

def parse_metadata_file(text: str) -> Optional[Dict[str, str]]:
    fp = re.search(r'^\s*file_path\s*=\s*"([^"]+)"\s*$', text, re.MULTILINE)
    si = re.search(r'^\s*start_index\s*=\s*([0-9]+)\s*$', text, re.MULTILINE)
    ei = re.search(r'^\s*end_index\s*=\s*([0-9]+)\s*$', text, re.MULTILINE)
    if not (fp and si and ei):
        return None
    return {"file_path": fp.group(1), "start_index": si.group(1), "end_index": ei.group(1)}

def iter_metadata_entries(metadata_dir: Path) -> Iterable[Tuple[str, int, int, Path]]:
    """Yield (file_path, start, end, source_file) for each metadata file under dir."""
    for p in sorted(metadata_dir.iterdir()):
        if p.is_dir():
            continue
        try:
            txt = p.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        parsed = parse_metadata_file(txt)
        if not parsed:
            continue
        yield (parsed["file_path"], int(parsed["start_index"]), int(parsed["end_index"]), p)

def ensure_header(csv_path: Path, header: List[str]) -> None:
    if not csv_path.exists():
        with csv_path.open("w", newline="", encoding="utf-8") as f:
            csv.writer(f).writerow(header)
        return

    # Validate exact header match (you guaranteed identical columns).
    with csv_path.open("r", newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        try:
            existing = next(reader)
        except StopIteration:
            existing = []
    if existing != header:
        sys.stderr.write(
            "[error] Existing CSV header does not match newly observed metrics.\n"
            "        Since columns must be identical per crate, refusing to append.\n"
        )
        sys.stderr.write(f"Existing: {existing}\nNew:      {header}\n")
        sys.exit(3)

def resolve_under_base(base_root: Path, file_path_str: str) -> Path:
    p = Path(file_path_str)
    return p if p.is_absolute() else (base_root / p).resolve()

def main():
    parser = argparse.ArgumentParser(
        description="Run rem-extract on selections and write ONE row per crate to a wide CSV (incremental metrics as columns)."
    )
    parser.add_argument("--rem-extract", type=Path, required=True,
        help="Path to rem-extract binary (e.g., ../target/release/rem-extract)")
    parser.add_argument("--csv", type=Path, default=Path("rem_extract_metrics.csv"),
        help="Output CSV (wide form, one row per crate)")
    parser.add_argument("--new-fn-name", default="extractedfn",
        help="New function name to use (default: extractedfn)")
    # Layout: evaluation/{scripts,testfiles}; metadata is under testfiles/REM_Original/0_METADATA
    parser.add_argument("--base-root", type=Path,
        default=(Path(__file__).resolve().parent.parent / "testfiles"),
        help="Base root that prefixes relative metadata file_path (default: evaluation/testfiles)")
    parser.add_argument("--metadata-dir", type=Path,
        default=(Path(__file__).resolve().parent.parent / "testfiles" / "REM_Original" / "0_METADATA"),
        help="Directory of metadata files (default: evaluation/testfiles/REM_Original/0_METADATA)")
    # Single-file mode (optional)
    parser.add_argument("--file", type=Path, help="Single file to run (absolute or under --base-root)")
    parser.add_argument("--start", type=int)
    parser.add_argument("--end", type=int)
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--json", dest="json_out", action="store_true")
    parser.add_argument("--no-metrics-flag", action="store_true", help="Do not pass --metrics")
    parser.add_argument("--extra", nargs=argparse.REMAINDER, help="Extra args forwarded to rem-extract")
    args = parser.parse_args()

    enable_metrics = not args.no_metrics_flag

    # Build task list
    tasks: List[Tuple[str, int, int, Optional[Path]]] = []
    if args.file and args.start is not None and args.end is not None:
        tasks.append((str(args.file), args.start, args.end, None))
    else:
        for fp, s, e, src in iter_metadata_entries(args.metadata_dir):
            tasks.append((fp, s, e, src))

    if not tasks:
        sys.stderr.write("No tasks found. Provide --file/--start/--end or ensure metadata dir is populated.\n")
        sys.exit(2)

    # First pass: run one task to discover metric columns (canonical order),
    # then write/validate header. Afterwards, append rows for all tasks.
    first_fp, first_s, first_e, _ = tasks[0]
    first_abs = resolve_under_base(args.base_root, first_fp)
    print(f"Discovering metric columns using: {first_abs} [{first_s},{first_e}]")
    code, out, err = run_rem_extract(
        args.rem_extract, first_abs, args.new_fn_name, first_s, first_e,
        enable_metrics, args.verbose, args.json_out, args.extra
    )
    if code != 0:
        sys.stderr.write(f"[warn] rem-extract exited with code {code} during discovery.\n")
        if err.strip():
            sys.stderr.write("---- stderr ----\n" + err + "\n")

    inc_metrics = parse_incremental_metrics(out)
    if not inc_metrics:
        sys.stderr.write("[error] No incremental metrics parsed (did you pass --metrics?).\n")
        sys.exit(4)

    # Derive canonical metric columns in encountered order
    metric_cols = [normalize_key(frm, to) for (frm, to, _) in inc_metrics]
    # Build and write header
    header = ["timestamp_iso", "file_path", "start_index", "end_index", "exit_code"] + metric_cols
    ensure_header(args.csv, header)

    # Helper: write one row
    def write_row(abs_file: Path, start_idx: int, end_idx: int,
                  incs: List[Tuple[str, str, float]], exit_code: int) -> None:
        ts = dt.datetime.now().isoformat(timespec="seconds")
        values: Dict[str, float] = {}
        for frm, to, secs in incs:
            values[normalize_key(frm, to)] = secs

        # Verify columns match exactly what header expects
        row = [
            ts,
            str(abs_file),
            start_idx,
            end_idx,
            exit_code,
        ]
        # Fill metric columns in header order
        for col in metric_cols:
            row.append(values.get(col, ""))  # should all exist; empty if not

        with args.csv.open("a", newline="", encoding="utf-8") as f:
            csv.writer(f).writerow(row)

    # We already ran the first task—record its row:
    write_row(first_abs, first_s, first_e, inc_metrics, code)

    # Remaining tasks
    for fp, s, e, src in tasks[1:]:
        abs_file = resolve_under_base(args.base_root, fp)
        print(f"Running: {abs_file} [{s},{e}]  (metadata: {src.name if src else 'single'})")
        code, out, err = run_rem_extract(
            args.rem_extract, abs_file, args.new_fn_name, s, e,
            enable_metrics, args.verbose, args.json_out, args.extra
        )
        if code != 0:
            sys.stderr.write(f"[warn] rem-extract exited with code {code}\n")
            if err.strip():
                sys.stderr.write("---- stderr ----\n" + err + "\n")

        incs = parse_incremental_metrics(out)
        # Validate column set matches discovery
        cols_this = [normalize_key(frm, to) for (frm, to, _) in incs]
        if cols_this != metric_cols:
            sys.stderr.write(
                "[error] Incremental metric set/order changed compared to discovery run.\n"
                "        You required identical columns; refusing to write a mismatched row.\n"
            )
            sys.stderr.write(f"Expected: {metric_cols}\nFound:    {cols_this}\n")
            sys.exit(5)

        write_row(abs_file, s, e, incs, code)

    print(f"\nDone. Wrote one row per crate to: {args.csv}")

if __name__ == "__main__":
    main()
