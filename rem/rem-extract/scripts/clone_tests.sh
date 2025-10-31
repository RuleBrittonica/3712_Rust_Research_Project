#!/usr/bin/env bash
set -euo pipefail

# Fixed configuration — NO customization
REPO_URL="https://github.com/RuleBrittonica/3712_Rust_Research_Project.git"
COMMIT_HASH="1acbdffab3f77c5821d68b81374b04343c97de45"
SUBPATH="rem/rem-extract/testdata"
DEST_DIR="$(cd "$(dirname "$0")/.." && pwd)/testdata"

echo "[i] Downloading $SUBPATH from commit $COMMIT_HASH"
echo "[i] Source repo: $REPO_URL"
echo "[i] Output folder: $DEST_DIR"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# minimal shallow partial clone, only fetch blobs when needed
git clone --quiet --filter=blob:none --no-checkout "$REPO_URL" "$TMP_DIR"

(
  cd "$TMP_DIR"

  git sparse-checkout init --cone
  git sparse-checkout set "$SUBPATH"

  git fetch --quiet --depth=1 origin "$COMMIT_HASH"
  git checkout --quiet --detach "$COMMIT_HASH"

  if [[ ! -d "$SUBPATH" ]]; then
    echo "[!] ERROR: $SUBPATH not found at commit $COMMIT_HASH" >&2
    exit 1
  fi

  rm -rf "$DEST_DIR"
  mkdir -p "$DEST_DIR"
  cp -a "$SUBPATH"/. "$DEST_DIR/"
)

echo "[✓] Testdata updated successfully!"
echo "[✓] Location: $DEST_DIR"
