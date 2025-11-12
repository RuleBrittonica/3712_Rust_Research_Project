#!/usr/bin/env bash
set -euo pipefail

# 0) Load config from config.toml
#    Exports: SWITCH, OCAML_VARIANT, OPAM_PACKAGES, AENEAS_REPO, AENEAS_REF
eval "$(python3 tools/cfg.py export-shell macos)"

# 1) Homebrew & Core Packages
echo "=> Checking for Homebrew…"
if ! command -v brew >/dev/null; then
  echo "   Homebrew not found. Installing…"
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.bash_profile
  eval "$(/opt/homebrew/bin/brew shellenv)"
fi

echo "=> Updating Homebrew…"
brew update

echo "=> Installing core packages…"
brew install \
  git \
  curl \
  pkg-config \
  openssl@3 \
  opam \
  dune \
  make

# ensure we pick up openssl@3
export LDFLAGS="-L$(brew --prefix openssl@3)/lib"
export CPPFLAGS="-I$(brew --prefix openssl@3)/include"

# 2) Rust Toolchain
if ! command -v rustup >/dev/null; then
  echo "=> Installing rustup…"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# TOOLCHAIN="nightly-2025-02-08"
TOOLCHAIN="nightly"
echo "=> Ensuring Rust toolchain $TOOLCHAIN…"
rustup toolchain install "$TOOLCHAIN" --profile minimal
for comp in rust-src rust-std rustc-dev llvm-tools-preview rust-analyzer-preview rustfmt; do
  rustup component add "$comp" --toolchain "$TOOLCHAIN"
done

# 3) OPAM / OCaml / Coq
echo "=> Initializing OPAM (if needed)…"
if ! opam config env >/dev/null 2>&1; then
  opam init -y --disable-sandboxing
fi
eval "$(opam env)"

echo "=> Setting up OPAM switch $SWITCH…"
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

echo "=> Updating OPAM…"
opam update --yes
opam upgrade --yes

echo "=> Installing OCaml libraries from config…"
# OPAM_PACKAGES is space-separated (from cfg.py)
opam install -y $OPAM_PACKAGES

# Coq install: keep behavior, but skip if on OCaml 5.x (8.18 may not be compatible)
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

# 4) Aeneas & CHARON (from config)
REPO="${AENEAS_REPO:?AENEAS_REPO not set by cfg.py}"
REF="${AENEAS_REF:-}"
CLONE="$HOME/.cache/aeneas"
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"

echo "=> Cloning/updating Aeneas…"
if [ ! -d "$CLONE/.git" ]; then
  git clone "$REPO" "$CLONE"
else
  git -C "$CLONE" remote set-url origin "$REPO"
  git -C "$CLONE" fetch --all --tags
fi

if [ -n "$REF" ]; then
  echo "=> Checking out Aeneas @ $REF…"
  git -C "$CLONE" fetch --all --tags
  git -C "$CLONE" checkout --detach "$REF" || {
    echo "!! Failed to checkout '$REF' — does it exist?" >&2
    exit 1
  }
else
  echo "=> No AENEAS_REF provided; staying on repository default."
  git -C "$CLONE" checkout -f "$(git -C "$CLONE" rev-parse HEAD)"
fi

echo "=> Building CHARON & Aeneas…"
cd "$CLONE"
make setup-charon
make
make test

echo "=> Installing binaries to $BIN_DIR…"
mkdir -p "$BIN_DIR"
for rel in bin/aeneas charon/bin/charon charon/bin/charon-driver; do
  if [ -f "$CLONE/$rel" ]; then
    cp -v "$CLONE/$rel" "$BIN_DIR"
    chmod +x "$BIN_DIR/$(basename "$rel")"
  else
    echo "Warning: $rel not found"
  fi
done

# 5) rem-server (parity with Linux script)
echo "=> Setting Rust default to $TOOLCHAIN…"
rustup default "$TOOLCHAIN"

if ! command -v rem-server >/dev/null; then
  echo "=> Installing rem-server…"
  cargo install rem-server --locked
else
  echo "=> rem-server already installed, skipping"
fi

echo
echo "Done! Make sure:"
echo '    export PATH="$HOME/.local/bin:$PATH"'
