import os
import csv
import time
import subprocess
from pathlib import Path

BASE = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple")
CSV_PATH = BASE.parent / "results.csv"

def timed_aeneas(llbc_file: Path, dest_dir: Path) -> float:
    cmd = [
        "aeneas",
        "-backend",
        "coq",
        str(llbc_file),
        "-dest",
        str(dest_dir),
        "-abort-on-error",
        "-soft-warnings"
    ]
    start = time.perf_counter()
    subprocess.run(cmd, check=True)
    end = time.perf_counter()
    return (end - start) * 1000

def process_case(case_dir: Path) -> float:
    equiv_dir = case_dir / "equiv"
    if not equiv_dir.is_dir():
        return 0.0

    llbc_files = list(equiv_dir.glob("*.llbc"))
    total_ms = 0.0

    for llbc in llbc_files:
        total_ms += timed_aeneas(llbc, equiv_dir)

    return total_ms

def main():
    existing_rows = []
    if CSV_PATH.exists():
        with open(CSV_PATH, "r") as f:
            reader = csv.DictReader(f)
            for row in reader:
                existing_rows.append(row)

    rows_by_name = {row["case_name"]: row for row in existing_rows}

    for case in BASE.iterdir():
        if not case.is_dir():
            continue
        ms = process_case(case)
        name = case.name
        if name in rows_by_name:
            rows_by_name[name]["coq"] = f"{ms:.2f}"

    with open(CSV_PATH, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=["case_name", "llbc_fresh", "llbc_cached", "coq", "equiv"]
        )
        writer.writeheader()
        for row in rows_by_name.values():
            writer.writerow(row)

    print("Done.")

if __name__ == "__main__":
    main()
