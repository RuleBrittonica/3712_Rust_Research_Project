import os
import csv
import time
import subprocess
from pathlib import Path

BASE = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple")
CSV_PATH = BASE.parent / "results.csv"
VERIFY_BIN = Path("/home/matt/3712_Rust_Research_Project/rem/target/release/rem-verification")

def timed_verification(case_dir: Path, case_name: str) -> float:
    """
    Run rem-verification on the given case_dir (expected to be .../equiv).
    Prints useful information and returns runtime in milliseconds.
    """

    print(f"\n=== Running verification for: {case_name} ===")

    input_v = case_dir / "Input.v"
    out_v = case_dir / "Out.v"
    top_level_file = case_dir / "top_level.txt"

    missing = []
    if not input_v.exists(): missing.append("Input.v")
    if not out_v.exists(): missing.append("Out.v")
    if not top_level_file.exists(): missing.append("top_level.txt")

    if missing:
        print(f"Skipping {case_name}: missing files: {', '.join(missing)}")
        return 0.0

    top_level_fn = top_level_file.read_text().strip()

    cmd = [
        str(VERIFY_BIN),
        "verify",
        str(input_v),
        str(out_v),
        top_level_fn,
        "--verbose"
    ]

    print("Command:", " ".join(cmd))

    start = time.perf_counter()
    try:
        result = subprocess.run(
            cmd,
            check=True,
            text=True,
            capture_output=True
        )
        end = time.perf_counter()

        print("Output:")
        print(result.stdout)

        ms = (end - start) * 1000.0
        print(f"Success: {ms:.2f} ms")

        return ms

    except subprocess.CalledProcessError as e:
        end = time.perf_counter()
        ms = (end - start) * 1000.0

        print("ERROR during verification:")
        print(e.stdout)
        print(e.stderr)
        print(f"Failed after {ms:.2f} ms")

        return ms


def main():
    # Load CSV
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

        name = case.name
        equiv_dir = case / "equiv"

        ms = timed_verification(equiv_dir, name)

        if name in rows_by_name:
            rows_by_name[name]["equiv"] = f"{ms:.2f}"
        else:
            print(f"WARNING: case '{name}' not found in CSV, skipping writing")

    with open(CSV_PATH, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=["case_name", "llbc_fresh", "llbc_cached", "coq", "equiv"]
        )
        writer.writeheader()
        for row in rows_by_name.values():
            writer.writerow(row)

    print("\nAll done.")


if __name__ == "__main__":
    main()
