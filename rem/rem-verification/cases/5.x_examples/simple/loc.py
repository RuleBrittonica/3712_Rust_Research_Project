#!/usr/bin/env python3
import os
import csv
import sys
from pathlib import Path

BASE_DIR = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple")
DEFAULT_CSV = BASE_DIR / "loc_report.csv"

def count_loc(path: Path) -> int | None:
    """Return number of lines in file, or None if it doesn't exist."""
    if not path.is_file():
        return None
    with path.open("r", encoding="utf-8") as f:
        return sum(1 for _ in f)


def main(base_dir: Path = BASE_DIR, out_csv: Path = DEFAULT_CSV) -> None:
    rows = []

    for entry in sorted(base_dir.iterdir()):
        if not entry.is_dir():
            continue

        pname = entry.name
        input_path = entry / "input" / "src"/ "main.rs"
        out_path = entry / "out" /"src" / "main.rs"

        input_loc = count_loc(input_path)
        out_loc = count_loc(out_path)

        if input_loc is None:
            print(f"[WARN] Missing input file: {input_path}", file=sys.stderr)
        if out_loc is None:
            print(f"[WARN] Missing out file: {out_path}", file=sys.stderr)

        rows.append({
            "project": pname,
            "input_main_rs_loc": input_loc if input_loc is not None else "",
            "out_main_rs_loc": out_loc if out_loc is not None else "",
        })

    # Write CSV
    with out_csv.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=["project", "input_main_rs_loc", "out_main_rs_loc"])
        writer.writeheader()
        writer.writerows(rows)

    print(f"Wrote LOC report to {out_csv}")


if __name__ == "__main__":
    base = Path(sys.argv[1]) if len(sys.argv) > 1 else BASE_DIR
    out = Path(sys.argv[2]) if len(sys.argv) > 2 else DEFAULT_CSV
    main(base, out)
