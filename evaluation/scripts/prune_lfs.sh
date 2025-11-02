#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-evaluation/testfiles}"
DRY_RUN="${DRY_RUN:-1}"

# Ensure we're in a Git repo
git rev-parse --show-toplevel >/dev/null 2>&1 || {
  echo "ERROR: Not inside a git repository." >&2
  exit 1
}

echo "=== Pruning LFS-tracked content under: $TARGET_DIR ==="
echo "Dry run: $DRY_RUN (set DRY_RUN=0 to actually delete)"
echo

removed=0
removed_bytes=0

remove_file() {
  local f="$1"
  local sz
  sz=$(stat -c%s -- "$f" 2>/dev/null || echo 0)

  if [ "$DRY_RUN" = "1" ]; then
    echo "Would remove: $f  ($(numfmt --to=iec "$sz" 2>/dev/null || echo "${sz}B"))"
  else
    git rm --cached --quiet -- "$f" 2>/dev/null || true
    rm -f -- "$f"
    echo "Removed: $f"
  fi

  removed=$((removed + 1))
  removed_bytes=$((removed_bytes + sz))
}

# 1) Primary: use git-lfs to list LFS-tracked files under TARGET_DIR
lfs_available=0
if command -v git-lfs >/dev/null 2>&1; then
  lfs_available=1
  mapfile -t LFS_FILES < <(git lfs ls-files -n "$TARGET_DIR" 2>/dev/null || true)
else
  LFS_FILES=()
fi

if [ "${#LFS_FILES[@]}" -gt 0 ]; then
  echo "--- git-lfs reports ${#LFS_FILES[@]} LFS-tracked file(s) ---"
  for f in "${LFS_FILES[@]}"; do
    [ -e "$f" ] || continue
    remove_file "$f"
  done
else
  if [ "$lfs_available" = "1" ]; then
    echo "--- git-lfs found no LFS-tracked files under $TARGET_DIR ---"
  else
    echo "--- git-lfs not installed; falling back to pointer detection ---"
  fi
fi


echo
echo "--- Scanning for LFS pointer files (text stubs) ---"
mapfile -d '' PTR_CANDIDATES < <(
  find "$TARGET_DIR" \
    -type f -size -1024c \
    -not -path "*/.git/*" \
    -print0 2>/dev/null
)

ptr_count=0
for f in "${PTR_CANDIDATES[@]:-}"; do
  if head -n1 -- "$f" 2>/dev/null | grep -q '^version https://git-lfs.github.com/spec/v1$'; then
    if sed -n '2p' -- "$f" 2>/dev/null | grep -q '^oid sha256:'; then
      remove_file "$f"
      ptr_count=$((ptr_count + 1))
    fi
  fi
done

echo
hr_total=$(numfmt --to=iec "$removed_bytes" 2>/dev/null || echo "${removed_bytes}B")
if [ "$DRY_RUN" = "1" ]; then
  echo "Dry run complete. Matches: $removed (≈ $hr_total)."
  echo "Re-run with: DRY_RUN=0 $0 $TARGET_DIR"
else
  echo "Deleted $removed item(s), ≈ $hr_total."
  echo "Now commit the changes"
fi
