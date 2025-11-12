#!/usr/bin/env bash
set -euo pipefail


# Usage:
#   ./import_repo https://github.com/<owner>/<repo>/commit/<sha>

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 https://github.com/<owner>/<repo>/commit/<sha>" >&2
  exit 1
fi

URL="$1"

# Accept these URL forms:
#  - https://github.com/owner/repo/commit/<sha>
#  - https://github.com/owner/repo/tree/<ref>  (e.g., tag or branch)  [fallback]
#  - https://github.com/owner/repo/.git        [fallback to HEAD]      [optional]
owner="" repo="" ref=""
if [[ "$URL" =~ github\.com/([^/]+)/([^/]+)/(commit|tree)/([^/?#]+) ]]; then
  owner="${BASH_REMATCH[1]}"
  repo="${BASH_REMATCH[2]}"
  ref="${BASH_REMATCH[4]}"
elif [[ "$URL" =~ github\.com/([^/]+)/([^/]+)(\.git)?/?$ ]]; then
  owner="${BASH_REMATCH[1]}"
  repo="${BASH_REMATCH[2]}"
  ref="HEAD"
else
  echo "Unrecognised URL. Expected a GitHub commit or tree URL." >&2
  exit 2
fi
repo="${repo%.git}"

ROOT="$(pwd)"
TMP_BASE="${ROOT}/.rem_import_tmp"
mkdir -p "$TMP_BASE"
SCRATCH="$(mktemp -d "${TMP_BASE}/tmp.XXXXXX")"

cleanup() {
  rm -rf "$SCRATCH" || true
}
trap cleanup EXIT

echo "==> Importing:"
echo "    repo: https://github.com/${owner}/${repo}"
echo "    ref : ${ref}"

# Shallow, partial clone; LFS disabled so we don't fetch large objects.
GIT_LFS_SKIP_SMUDGE=1 \
git -c filter.lfs.smudge= -c filter.lfs.process= -c filter.lfs.required=false \
  clone \
    --filter=blob:none \
    --no-tags \
    --depth=1 \
    --recurse-submodules=no \
    --quiet \
    "https://github.com/${owner}/${repo}.git" \
    "${SCRATCH}/repo"

pushd "${SCRATCH}/repo" >/dev/null

# Try to resolve the ref locally; if missing, fetch exactly that object.
if git rev-parse --verify --quiet "${ref}^{commit}" >/dev/null; then
  git checkout --detach --quiet "${ref}"
else
  # Fetch the specific commit/ref with depth 1 (works for SHAs and refs)
  git fetch --depth=1 --quiet origin "${ref}" || {
    echo "Failed to fetch ref/commit '${ref}' from origin." >&2
    exit 3
  }
  git checkout --detach --quiet FETCH_HEAD
fi

COMMIT_SHA="$(git rev-parse --verify --short=12 HEAD)"
popd >/dev/null

DEST="${ROOT}/${repo}-${COMMIT_SHA}"
if [[ -e "$DEST" ]]; then
  echo "Destination '${DEST}' already exists; refusing to overwrite." >&2
  exit 4
fi

mkdir -p "$DEST"

# Copy working tree files without git metadata.
rsync -a \
  --exclude ".git" \
  --exclude ".gitmodules" \
  --exclude ".github" \
  "${SCRATCH}/repo/" "${DEST}/"

# Detect & neutralise LFS pointer files (leave placeholders + manifest)
echo "==> Scanning for Git LFS pointer files..."
mapfile -t POINTERS < <(grep -IRl --null \
  --exclude-dir=".git" --exclude-dir=".github" \
  -m1 -e '^version https://git-lfs.github.com/spec/v1$' "${DEST}" | tr '\0' '\n' || true)

if (( ${#POINTERS[@]} > 0 )); then
  echo "    Found ${#POINTERS[@]} LFS pointer(s)."
  MANIFEST="${DEST}/.lfs_pointers_manifest.txt"
  : > "${MANIFEST}"
  for f in "${POINTERS[@]}"; do
    rel="${f#${DEST}/}"
    echo "${rel}" >> "${MANIFEST}"
    # Replace with empty file to avoid committing pointers into your repo
    : > "${f}"
  done
  echo "    Replaced pointers with 0-byte placeholders; list in ${MANIFEST}"
else
  echo "    No LFS pointers detected."
fi

# Provenance
cat > "${DEST}/.rem_import_provenance.txt" <<EOF
Imported from: https://github.com/${owner}/${repo}
Requested URL: ${URL}
Captured commit: ${COMMIT_SHA}
Imported at: $(date -Iseconds)
Tool: import_repo (one-arg vendor)
EOF

echo
echo "==> Done."
echo "    Captured commit: ${COMMIT_SHA}"
echo "    Output directory: ${DEST}"
echo "    Now: git add '${DEST}'"
