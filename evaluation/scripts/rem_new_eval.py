#!/usr/bin/env python3
import argparse
import csv
import datetime as dt
import re
import subprocess
import sys
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Tuple

SAVE_LOCATION = "/home/matt/3712_Rust_Research_Project/evaluation/Results/rem_new_extract_metrics.csv"

# Accept "inc:... <number> ns" (integer or float, we'll round to nearest ns int)
INC_RE = re.compile(r"^inc:(.+?)->(.+?)\s+([0-9]+(?:\.[0-9]+)?)\s*ns\s*$")

def normalize_key(frm: str, to: str) -> str:
    def norm(s: str) -> str:
        return re.sub(r"[^a-z0-9]+", "_", s.strip().lower()).strip("_")
    return f"{norm(frm)}__to__{norm(to)}"

def parse_incremental_metrics(stdout: str) -> List[Tuple[str, str, int]]:
    """Return (from, to, nanos) for each inc: line."""
    out: List[Tuple[str, str, int]] = []
    for line in stdout.splitlines():
        if not line.startswith("inc:"):
            continue
        m = INC_RE.match(line.strip())
        if m:
            frm = m.group(1).strip()
            to = m.group(2).strip()
            ns = int(round(float(m.group(3))))
            out.append((frm, to, ns))
    return out

def run_rem_extract(
    rem_extract: Path, file_path: Path, new_fn_name: str, start_index: int, end_index: int,
    verbose: bool = False, json_out: bool = False,
    extra_args: Optional[List[str]] = None, cwd: Optional[Path] = None
) -> Tuple[int, str, str]:
    cmd = [
        str(rem_extract), "extract", str(file_path),
        str(new_fn_name), str(start_index), str(end_index),
        "--metrics"
    ]
    if verbose:
        cmd.append("--verbose")
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

    with csv_path.open("r", newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        try:
            existing = next(reader)
        except StopIteration:
            existing = []
    if existing != header:
        sys.stderr.write(
            "[error] Existing CSV header does not match newly observed metrics.\n"
            "        Refusing to append to avoid corrupting an existing dataset.\n"
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
    parser.add_argument("--csv", type=Path, default=Path(SAVE_LOCATION),
        help="Output CSV (wide form, one row per crate)")
    parser.add_argument("--new-fn-name", default="extractedfn",
        help="New function name to use (default: extractedfn)")
    parser.add_argument("--base-root", type=Path,
        default=(Path(__file__).resolve().parent.parent / "testfiles"),
        help="Base root that prefixes relative metadata file_path (default: evaluation/testfiles)")
    parser.add_argument("--metadata-dir", type=Path,
        default=(Path(__file__).resolve().parent.parent / "testfiles" / "REM_New" / "eval_cases" / "0_METADATA"),
        help="Directory of metadata files (default: evaluation/testfiles/REM_New/eval_cases/0_METADATA)")
    parser.add_argument("--file", type=Path, help="Single file to run (absolute or under --base-root)")
    parser.add_argument("--start", type=int)
    parser.add_argument("--end", type=int)
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("--json", dest="json_out", action="store_true")
    parser.add_argument("--no-metrics-flag", action="store_true", help="Do not pass --metrics")
    parser.add_argument("--extra", nargs=argparse.REMAINDER, help="Extra args forwarded to rem-extract")
    args = parser.parse_args()

    enable_metrics = not args.no_metrics_flag

    tasks: List[Tuple[str, int, int, Optional[Path]]] = []
    if args.file and args.start is not None and args.end is not None:
        tasks.append((str(args.file), args.start, args.end, None))
    else:
        for fp, s, e, src in iter_metadata_entries(args.metadata_dir):
            tasks.append((fp, s, e, src))

    if not tasks:
        sys.stderr.write("No tasks found. Provide --file/--start/--end or ensure metadata dir is populated.\n")
        sys.exit(2)

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
        sys.stderr.write(f"Output was:\n{out}\n")
        sys.exit(4)

    metric_cols = [normalize_key(frm, to) for (frm, to, _) in inc_metrics]

    header = ["timestamp_iso", "file_path", "start_index", "end_index", "exit_code"] + metric_cols
    ensure_header(args.csv, header)

    def write_row(abs_file: Path, start_idx: int, end_idx: int,
                  incs: List[Tuple[str, str, int]], exit_code: int) -> None:
        ts = dt.datetime.now().isoformat(timespec="seconds")
        values: Dict[str, int] = {}
        for frm, to, nanos in incs:
            values[normalize_key(frm, to)] = nanos

        row: List[object] = [
            ts,
            str(abs_file),
            start_idx,
            end_idx,
            exit_code,
        ]
        for col in metric_cols:
            row.append(values.get(col, 0))

        with args.csv.open("a", newline="", encoding="utf-8") as f:
            csv.writer(f).writerow(row)

    # Record first run
    write_row(first_abs, first_s, first_e, inc_metrics, code)

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

        write_row(abs_file, s, e, incs, code)

    print(f"\nDone. Wrote one row per crate to: {SAVE_LOCATION}")

if __name__ == "__main__":
    main()
