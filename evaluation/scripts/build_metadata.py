#!/usr/bin/env python3
import os
from pathlib import Path
from typing import Optional, Tuple


TESTFILES_ROOT = Path("evaluation/testfiles")
REM_ORIGINAL = TESTFILES_ROOT / "REM_Original"
METADATA_DIR = REM_ORIGINAL / "0_METADATA"

START_MARKERS = [
    b"// START SELECTION //",
    b"/* START SELECTION */",
]
END_MARKERS = [
    b"// END SELECTION //",
    b"/* END SELECTION */",
]


def find_first_marker_pair(content: bytes) -> Tuple[int, int]:
    """
    Find the first START marker (any style), then the first END marker (any style)
    that occurs *after* that START. Return byte offsets:
      - start_index: index of the last '/' of the START marker
      - end_index: index of the first '/' of the END marker
    Raises ValueError if not found.
    """
    # Find earliest occurrence among all start markers
    start_choice: Optional[Tuple[int, bytes]] = None  # (pos, marker)
    for m in START_MARKERS:
      pos = content.find(m)
      if pos != -1 and (start_choice is None or pos < start_choice[0]):
        start_choice = (pos, m)

    if start_choice is None:
        raise ValueError("START marker not found")

    start_pos, start_marker = start_choice
    # last '/' of the start marker is simply the last byte of the marker,
    # because both patterns end with '/' (either ...'//' or '*/').
    start_index = start_pos + len(start_marker) - 1

    # Find the earliest END marker that appears AFTER the chosen start
    end_choice: Optional[int] = None
    for em in END_MARKERS:
        pos = content.find(em, start_pos + len(start_marker))
        if pos != -1 and (end_choice is None or pos < end_choice):
            end_choice = pos

    if end_choice is None:
        raise ValueError("END marker not found after START")

    end_index = end_choice  # first '/' of the end marker (both forms start with '/')

    return start_index, end_index


def nearest_cargo_toml(file_path: Path) -> Optional[Path]:
    """
    Walk upward from file_path's directory to find the nearest Cargo.toml.
    """
    cur = file_path.parent
    while True:
        cand = cur / "Cargo.toml"
        if cand.exists():
            return cand
        if cur.parent == cur:
            return None
        cur = cur.parent


def write_toml(out_path: Path, file_rel: Path, cargo_rel: Path, start_idx: int, end_idx: int) -> None:
    out_path.parent.mkdir(parents=True, exist_ok=True)
    with out_path.open("w", encoding="utf-8", newline="\n") as f:
        f.write("# Auto-generated selection metadata\n")
        f.write(f'file_path = "{file_rel.as_posix()}"\n')
        f.write(f'cargo_path = "{cargo_rel.as_posix()}"\n')
        f.write(f"start_index = {start_idx}\n")
        f.write(f"end_index = {end_idx}\n")


def process_project_dir(project_dir: Path) -> None:
    """
    Search this top-level project directory for the unique selection markers.
    Once found, write <name>.toml into 0_METADATA and stop.
    """
    project_name = project_dir.name

    for root, _, files in os.walk(project_dir):
        for name in files:
            if not name.endswith(".rs"):
                continue
            rs_path = Path(root) / name

            try:
                content = rs_path.read_bytes()
            except Exception:
                continue

            try:
                start_idx, end_idx = find_first_marker_pair(content)
            except ValueError:
                continue  # not in this file

            file_rel = rs_path.relative_to(TESTFILES_ROOT)
            cargo = nearest_cargo_toml(rs_path)
            if cargo is None:
                raise RuntimeError(f"No Cargo.toml found above {rs_path}")
            cargo_rel = cargo.relative_to(TESTFILES_ROOT)

            out_toml = METADATA_DIR / f"{project_name}.toml"
            write_toml(out_toml, file_rel, cargo_rel, start_idx, end_idx)

            print(f"[ok] {project_name}:")
            print(f"     file_path  = {file_rel}")
            print(f"     cargo_path = {cargo_rel}")
            print(f"     start_index= {start_idx}")
            print(f"     end_index  = {end_idx}")
            return  # exactly one selection per dir

    print(f"[warn] {project_name}: no selection markers found")


def main() -> None:
    if not REM_ORIGINAL.exists():
        raise SystemExit(f"Missing directory: {REM_ORIGINAL}")

    for entry in sorted(REM_ORIGINAL.iterdir()):
        if not entry.is_dir():
            continue
        if entry.name == "0_METADATA":
            continue  # skip metadata dir
        process_project_dir(entry)


if __name__ == "__main__":
    main()