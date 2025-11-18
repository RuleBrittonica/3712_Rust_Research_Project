import os
import csv
import time
import shutil
import subprocess
from pathlib import Path

BASE = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple")
CSV_PATH = BASE.parent / "results.csv"

def timed_charon(crate_src: Path, llbc_out: Path) -> float:
    llbc_out.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        "charon",
        "cargo",
        "--preset=aeneas",
        "--dest-file",
        str(llbc_out)
    ]
    start = time.perf_counter()
    subprocess.run(cmd, cwd=crate_src, check=True)
    end = time.perf_counter()
    return (end - start) * 1000

def delete_target(crate_dir: Path):
    t = crate_dir / "target"
    if t.exists():
        shutil.rmtree(t)

def delete_llbc(equiv_dir: Path):
    if equiv_dir.exists():
        for f in equiv_dir.glob("*.llbc"):
            f.unlink()

def process_case(case_dir: Path) -> dict:
    print(f"\n=== Processing case: {case_dir.name} ===")
    input_dir = case_dir / "input"
    out_dir = case_dir / "out"
    if not input_dir.is_dir() or not out_dir.is_dir():
        return None

    equiv_dir = case_dir / "equiv"
    equiv_dir.mkdir(exist_ok=True)
    delete_llbc(equiv_dir)

    print("\n--- FRESH RUN ---")
    delete_target(input_dir)
    delete_target(out_dir)

    input_src = input_dir / "src"
    out_src = out_dir / "src"

    fresh_total = 0.0
    if input_src.is_dir():
        fresh_total += timed_charon(input_src, equiv_dir / "input_fresh.llbc")
    if out_src.is_dir():
        fresh_total += timed_charon(out_src, equiv_dir / "out_fresh.llbc")

    print("\n--- CACHED RUN ---")
    delete_llbc(equiv_dir)

    cached_total = 0.0
    if input_src.is_dir():
        cached_total += timed_charon(input_src, equiv_dir / "input.llbc")
    if out_src.is_dir():
        cached_total += timed_charon(out_src, equiv_dir / "out.llbc")

    return {
        "case_name": case_dir.name,
        "llbc_fresh": f"{fresh_total:.2f}",
        "llbc_cached": f"{cached_total:.2f}",
        "coq": "",
        "equiv": ""
    }

def main():
    rows = []
    for case in BASE.iterdir():
        if case.is_dir():
            row = process_case(case)
            if row:
                rows.append(row)

    with open(CSV_PATH, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=["case_name", "llbc_fresh", "llbc_cached", "coq", "equiv"]
        )
        writer.writeheader()
        writer.writerows(rows)

    print("\nDone.")

if __name__ == "__main__":
    main()
