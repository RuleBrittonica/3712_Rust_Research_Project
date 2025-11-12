#!/usr/bin/env bash
set -euo pipefail

# 0) Load config from config.toml
#    Exports: SWITCH, OCAML_VARIANT, OPAM_PACKAGES, AENEAS
eval "$(python3 tools/cfg.py export-shell linux)"

# 1) Setup build environment

echo "=> Updating apt repositories…"
sudo apt-get update

echo "=> Installing core system packages…"
sudo apt-get install -y \
    build-essential \
    git \
    curl \
    libssl-dev \
    pkg-config \
    opam \
    dune

# 2) Install rustup (hopefully you already have it!) and ensure correct toolchain
if ! command -v rustup >/dev/null 2>&1; then
  echo "=> Installing rustup…"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# TOOLCHAIN="nightly-2025-02-08"
TOOLCHAIN="nightly"
echo "=> Installing Rust toolchain $TOOLCHAIN…"
rustup toolchain install "$TOOLCHAIN" --profile minimal
for comp in rust-src rust-std rustc-dev llvm-tools-preview rust-analyzer-preview rustfmt; do
  rustup component add "$comp" --toolchain "$TOOLCHAIN"
done

# 3) Ensure OPAM is initialized and install CoQ dependencies
if ! opam config env >/dev/null 2>&1; then
  echo "=> Initializing OPAM…"
  opam init -y --disable-sandboxing
fi
eval "$(opam env)"

echo "=> Creating/updating OPAM switch $SWITCH…"
if ! opam switch list --short | grep -qx "$SWITCH"; then
  if [ -n "${OCAML_VARIANT:-}" ]; then
    opam switch create "$SWITCH" "$OCAML_VARIANT" --yes
  else
    opam switch create "$SWITCH" --yes
  fi
else
  opam switch "$SWITCH"
fi
eval "$(opam env)"

echo "=> Updating OPAM packages…"
opam update --yes
opam upgrade --yes --verbose

echo "=> Installing OCaml libraries from config…"
# OPAM_PACKAGES is space-separated (from cfg.py)
opam install -y $OPAM_PACKAGES

# Coq install: keep original behavior, but skip if on OCaml 5.x since 8.18 may not be compatible
REQ_COQ="8.18.0"
OCAML_MAJOR="$(ocamlc -version | cut -d. -f1)"
if [ "$OCAML_MAJOR" -ge 5 ]; then
  echo "=> Detected OCaml $(ocamlc -version). Skipping Coq $REQ_COQ (may be incompatible with OCaml 5.x)."
else
  if ! coqc --version 2>/dev/null | grep -qE "version[[:space:]]+$REQ_COQ"; then
    echo "=> Installing Coq $REQ_COQ…"
    opam pin add coq "$REQ_COQ" --yes --no-action
    opam install -y coq
  fi
fi

# 4) Install Aeneas and CHARON

# This is a pinned version of aeneas that is known to work with the current
# setup.It has a couple of changes to the Mkaefile to remove Nix compatibility
# but make it easier to build across the board. Feel free to install from source
# but YMMV.
REPO="${AENEAS_REPO:?AENEAS_REPO not set by cfg.py}"
REF="${AENEAS_REF:-}"
CLONE_DIR="$HOME/.cache/aeneas"
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"

echo "=> Cloning/updating Aeneas repo…"
if [ ! -d "$CLONE_DIR/.git" ]; then
  git clone "$REPO" "$CLONE_DIR"
else
  git -C "$CLONE_DIR" remote set-url origin "$REPO"
  git -C "$CLONE_DIR" fetch --all --tags
fi

if [ -n "$REF" ]; then
  echo "=> Checking out Aeneas @ $REF…"
  git -C "$CLONE_DIR" fetch --all --tags
  # try tag/branch/commit uniformly; detached is fine for builds
  git -C "$CLONE_DIR" checkout --detach "$REF" || {
    echo "!! Failed to checkout '$REF' — does it exist?" >&2
    exit 1
  }
else
  echo "=> No AENEAS_REF provided; staying on repository default."
  git -C "$CLONE_DIR" checkout -f "$(git -C "$CLONE_DIR" rev-parse HEAD)"
fi

echo "=> Building CHARON…"
cd "$CLONE_DIR"
make setup-charon

echo "=> Building Aeneas…"
make

echo "=> Running Aeneas install tests..."
make test

mkdir -p "$BIN_DIR"
for rel in bin/aeneas charon/bin/charon charon/bin/charon-driver; do
  if [ -f "$CLONE_DIR/$rel" ]; then
    echo "    -> Installing $(basename "$rel") to $BIN_DIR"
    cp "$CLONE_DIR/$rel" "$BIN_DIR"
    chmod +x "$BIN_DIR/$(basename "$rel")"
  else
    echo "Warning: $rel not found"
  fi
done

# 5) Ensure rem-server is installed
echo "=> Switching Rust default to $TOOLCHAIN…"
rustup default "$TOOLCHAIN"

if ! command -v rem-server >/dev/null 2>&1; then
  echo "=> Installing rem-server via cargo…"
  cargo install rem-server --locked
else
  echo "=> rem-server already present (skipping)"
fi

echo "All done! Make sure \$HOME/.local/bin is on your PATH:"
echo '    export PATH="$HOME/.local/bin:$PATH"'