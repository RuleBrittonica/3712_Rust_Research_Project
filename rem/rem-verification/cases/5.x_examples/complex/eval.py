#!/usr/bin/env python3
import csv
import shutil
import subprocess
import time
from pathlib import Path


COMPLEX_BASE = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/complex")

TOP_LEVEL_FNS_FILE = COMPLEX_BASE / "top_level_fns.txt"

ORIG_CRATE_DIR = COMPLEX_BASE / "business_logic"
REFAC_CRATE_DIR = COMPLEX_BASE / "business_logic_refactored"

VERIFY_ROOT = COMPLEX_BASE / "verify"
RESULTS_CSV = COMPLEX_BASE / "results.csv"

VERIFY_BIN = Path("/home/matt/3712_Rust_Research_Project/rem/target/release/rem-verification")
PRIMITIVES_V = Path("/home/matt/3712_Rust_Research_Project/rem/rem-verification/src/Primitives.v")


CHARON_CMD = ["charon", "cargo", "--preset=aeneas"]
AENEAS_CMD_BASE = ["aeneas", "-backend", "coq", "-abort-on-error", "-soft-warnings"]


def run_timed(cmd, cwd=None, label=""):
    print("="*80)
    if label:
        print(f"[{label}]")
    print("CMD:", " ".join(str(c) for c in cmd))
    start = time.perf_counter()
    result = subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        text=True,
        capture_output=True,
        check=True
    )
    end = time.perf_counter()
    ms = (end - start) * 1000.0
    print(result.stdout)
    if result.stderr:
        print("---stderr---")
        print(result.stderr)
    print(f"{label}: {ms:.2f} ms")
    return ms


def clean_targets():
    for crate in (ORIG_CRATE_DIR, REFAC_CRATE_DIR):
        t = crate / "target"
        if t.exists():
            print(f"Removing {t}")
            shutil.rmtree(t)


def run_verifyer(input_v: Path, out_v: Path, top_fn: str) -> float:
    cmd = [
        str(VERIFY_BIN),
        "verify",
        str(input_v),
        str(out_v),
        top_fn,
        "--verbose"
    ]
    print("Running verifyer:", " ".join(cmd))
    start = time.perf_counter()
    result = subprocess.run(cmd, text=True, capture_output=True, check=True)
    end = time.perf_counter()

    print(result.stdout)
    if result.stderr:
        print(result.stderr)

    return (end - start) * 1000.0


def main():
    VERIFY_ROOT.mkdir(exist_ok=True)

    with open(TOP_LEVEL_FNS_FILE) as f:
        top_fns = [l.strip() for l in f if l.strip()]

    if not RESULTS_CSV.exists():
        with open(RESULTS_CSV, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow([
                "fn",
                "charon_fresh_ms",
                "charon_cached_ms",
                "aeneas_ms",
                "verifyer_ms"
            ])

    for fn in top_fns:
        print("\n" + "#"*80)
        print(f"### Evaluating {fn} ###")

        verify_dir = VERIFY_ROOT / fn
        if verify_dir.exists():
            shutil.rmtree(verify_dir)
        verify_dir.mkdir(parents=True)

        shutil.copy2(PRIMITIVES_V, verify_dir / "Primitives.v")


        orig_llbc = verify_dir / "business_logic.llbc"
        refac_llbc = verify_dir / "business_logic_refactored.llbc"

        # clean targets to force rebuild (gives us a fresh run)
        clean_targets()
        fresh_ms = run_timed(
            CHARON_CMD + ["--dest-file", str(orig_llbc)],
            cwd=ORIG_CRATE_DIR,
            label=f"charon fresh original {fn}"
        )
        fresh_ms += run_timed(
            CHARON_CMD + ["--dest-file", str(refac_llbc)],
            cwd=REFAC_CRATE_DIR,
            label=f"charon fresh refactored {fn}"
        )

        cached_ms = run_timed(
            CHARON_CMD + ["--dest-file", str(orig_llbc)],
            cwd=ORIG_CRATE_DIR,
            label=f"charon cached original {fn}"
        )
        cached_ms += run_timed(
            CHARON_CMD + ["--dest-file", str(refac_llbc)],
            cwd=REFAC_CRATE_DIR,
            label=f"charon cached refactored {fn}"
        )

        aeneas_ms = run_timed(
            AENEAS_CMD_BASE + ["-dest", str(verify_dir), str(orig_llbc)],
            cwd=verify_dir,
            label=f"aeneas orig {fn}"
        )
        aeneas_ms += run_timed(
            AENEAS_CMD_BASE + ["-dest", str(verify_dir), str(refac_llbc)],
            cwd=verify_dir,
            label=f"aeneas refac {fn}"
        )

        input_v = verify_dir / "BusinessLogic.v"
        out_v = verify_dir / "BusinessLogicRefactored.v"

        if not input_v.exists():
            raise FileNotFoundError(f"Expected Coq file not found: {input_v}")
        if not out_v.exists():
            raise FileNotFoundError(f"Expected Coq file not found: {out_v}")

        verify_ms = run_verifyer(input_v, out_v, fn)

        with open(RESULTS_CSV, "a", newline="") as f:
            writer = csv.writer(f)
            writer.writerow([
                fn,
                f"{fresh_ms:.3f}",
                f"{cached_ms:.3f}",
                f"{aeneas_ms:.3f}",
                f"{verify_ms:.3f}"
            ])

        print(f"--- Done with {fn} ---")

    print("\nAll done! Results in:", RESULTS_CSV)

if __name__ == "__main__":
    main()