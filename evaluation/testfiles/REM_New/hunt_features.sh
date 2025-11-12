#!/usr/bin/env bash
set -euo pipefail

# Features: generics, async, const, nlcf, hrtb, dyn
#
# Usage:
#   ./hunt_features.sh
#   ./hunt_features.sh --feature async
#   ./hunt_features.sh --repo deno-093f3ba  # prefix/substring OK
#
# Requires: ast-grep (sg), ripgrep (rg). bat optional.

RESULTS_DIR="results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

FEATURES=(generics async const nlcf hrtb dyn)

# helper: ensure tool present
need() { command -v "$1" >/dev/null 2>&1 || { echo "Missing tool: $1" >&2; exit 2; }; }
need rg
need sg

# repo discovery
discover_repos() {
  # children dirs (skip .git, results) that contain at least one *.rs (excluding target/.git)
  find . -mindepth 1 -maxdepth 1 -type d ! -name ".git" ! -name "results" -print0 \
  | while IFS= read -r -d '' d; do
      if find "$d" -type f -name "*.rs" -not -path "*/.git/*" -not -path "*/target/*" -print -quit | grep -q .; then
        echo "$d"
      fi
    done
}

# ast-grep wrappers (stderr muted, never fail)
SG="sg run -l rust"
sgq() { $SG -p "$1" "$2" 2>/dev/null || true; }

sg_matches_generics() {
  sgq 'fn $NAME<$T>(...) { ... }'          "$1"
  sgq 'struct $NAME<$T> { ... }'           "$1"
  sgq 'enum $NAME<$T> { ... }'             "$1"
  sgq 'impl<$T> $TY { ... }'               "$1"
  sgq 'impl $TY for $TY2<$T> { ... }'      "$1"
  sgq 'fn $NAME(...) where $COND { ... }'  "$1"
}

sg_matches_async() {
  sgq 'async fn $NAME(...) { ... }'        "$1"
  sgq 'async move { ... }'                 "$1"
  sgq '$EXPR.await'                        "$1"
  sgq 'fn $NAME(...) -> impl Future { ... }' "$1"
}

sg_matches_const() {
  sgq 'const fn $NAME(...) { ... }'        "$1"
  sgq 'const $NAME: $TY = $VAL;'           "$1"   # top-level const requires type
  sgq 'impl $T { const $NAME: $TY = $VAL; }' "$1" # associated const requires type
}

sg_matches_nlcf() {
  sgq 'match $EXPR { ... }'                "$1"
  sgq 'loop { ... }'                       "$1"
  sgq 'while $COND { ... }'                "$1"
  sgq 'for $PAT in $ITER { ... }'          "$1"
  sgq 'return $EXPR'                       "$1"
  sgq 'break'                              "$1"
  sgq 'continue'                           "$1"
}

sg_matches_dyn() {
  sgq 'dyn $Trait'                         "$1"
  sgq 'Box<dyn $Trait>'                    "$1"
  sgq 'Arc<dyn $Trait>'                    "$1"
  sgq 'Rc<dyn $Trait>'                     "$1"
}

# HRTB: reliable text fallback (various placements)
rg_matches_hrtb() {
  rg --pcre2 -n --no-heading -S \
     -g '!**/.git/**' -g '!**/target/**' -g '**/*.rs' \
     "for<'[a-z](?:\\s*,\\s*'[a-z])*>|where\\s+.*for<'[a-z]" \
     "$1" || true
}

# args
ONLY_FEATURE=""
ONLY_REPO=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --feature) ONLY_FEATURE="${2:-}"; shift 2;;
    --repo)    ONLY_REPO="${2:-}"; shift 2;;
    -h|--help)
      echo "Usage: $0 [--feature <generics|async|const|nlcf|hrtb|dyn>] [--repo <dir-substring>]"
      exit 0;;
    *) echo "Unknown arg: $1" >&2; exit 2;;
  esac
done

if [[ -n "$ONLY_FEATURE" ]]; then
  case " ${FEATURES[*]} " in
    *" $ONLY_FEATURE "*) ;; *)
      echo "Unknown feature: $ONLY_FEATURE" >&2; exit 3;;
  esac
fi

echo "Results -> $RESULTS_DIR"
SUMMARY="$RESULTS_DIR/summary_counts.txt"
: > "$SUMMARY"

# core scan
scan_one_feature_repo() {
  local feature="$1" repo="$2"
  local csv="$RESULTS_DIR/matches_${feature}.csv"
  local tmp="$RESULTS_DIR/.tmp_${feature}.txt"
  local files_out="$RESULTS_DIR/files_${feature}.txt"

  [[ -f "$csv" ]] || echo "feature,repo,file,line,snippet" > "$csv"
  : > "$tmp"

  case "$feature" in
    generics) sg_matches_generics  "$repo" ;;
    async)    sg_matches_async     "$repo" ;;
    const)    sg_matches_const     "$repo" ;;
    nlcf)     sg_matches_nlcf      "$repo" ;;
    dyn)      sg_matches_dyn       "$repo" ;;
    hrtb)     rg_matches_hrtb      "$repo" ;;
  esac | sed "s#^\./##" \
     | awk -F: -v feat="$feature" -v rep="$repo" '
         BEGIN{OFS=","}
         NF>=2 {
           file=$1; line=$2;
           $1=""; $2="";
           sub(/^::/,"");
           snippet=$0; gsub(/^, /,"",snippet);
           gsub(/"/,"\"\"",snippet);
           print feat, rep, file, line, "\"" snippet "\""
           print file >> "'"$tmp"'"
         }
       '

  if [[ -s "$tmp" ]]; then
    if [[ -f "$files_out" ]]; then
      cat "$tmp" >> "$files_out"
    else
      cp "$tmp" "$files_out"
    fi
  fi
}

# repos
mapfile -t REPOS < <(discover_repos)
if [[ -n "$ONLY_REPO" ]]; then
  # substring match (case-insensitive)
  mapfile -t REPOS < <(printf "%s\n" "${REPOS[@]}" | grep -i -- "$ONLY_REPO" || true)
fi
if [[ ${#REPOS[@]} -eq 0 ]]; then
  echo "No repos with .rs files found under $(pwd)" >&2
  exit 4
fi

for repo in "${REPOS[@]}"; do
  for feat in "${FEATURES[@]}"; do
    [[ -n "$ONLY_FEATURE" && "$ONLY_FEATURE" != "$feat" ]] && continue
    scan_one_feature_repo "$feat" "$repo"
  done
done

# rank + summarize
for feat in "${FEATURES[@]}"; do
  files_out="$RESULTS_DIR/files_${feat}.txt"
  if [[ -f "$files_out" ]]; then
    sort "$files_out" | uniq -c | sort -nr > "$files_out.ranked"
    mv "$files_out.ranked" "$files_out"
    total=$(awk '{s+=$1} END{print (s?s:0)}' "$files_out")
    files=$(wc -l < "$files_out")
    printf "%-8s %6s matches | %s files\n" "$feat" "$total" "$files" | tee -a "$SUMMARY"
  else
    printf "%-8s %6s matches | %s files\n" "$feat" "0" "0" | tee -a "$SUMMARY"
  fi
done

echo
echo "Summary:"
cat "$SUMMARY"
echo
echo "Per-feature ranked files: $RESULTS_DIR/files_<feature>.txt"
echo "Per-feature CSVs with file:line snippets: $RESULTS_DIR/matches_<feature>.csv"
