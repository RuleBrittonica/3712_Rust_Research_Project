#!/usr/bin/env python3
from __future__ import annotations
import argparse
import csv
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, List, Dict, Tuple, Set

DEFAULT_REPOS = [
    "denoland/deno",
    "tauri-apps/tauri",
    "rustdesk/rustdesk",
    "unionlabs/union",
    "FuelLabs/sway",
    "zed-industries/zed",
    "alacritty/alacritty",
    "rust-lang/rustlings",
    "FuelLabs/fuel-core",
    "astral-sh/uv",
    "lencx/ChatGPT",
    "sharkdp/bat",
    "BurntSushi/ripgrep",
    "meilisearch/meilisearch",
    "rust-unofficial/awesome-rust",
    "starship/starship",
    "dani-garcia/vaultwarden",
    "typst/typst",
]

GREP_PATTERNS = [
    r"extract",
    r"extract method",
    r"extract function",
    r"extract\s+.*into\s+.*function",
    r"factor out",
    r"pull out",
    r"refactor:.*extract",
    r"refactor.*extract",
]

CAPABILITY_REGEXES: Dict[str, List[re.Pattern]] = {
    "async_await": [
        re.compile(r"^\+.*\basync\b", re.IGNORECASE),
        re.compile(r"^\+.*\.await\b", re.IGNORECASE),
        re.compile(r"^\+.*->\s*impl\s+Future", re.IGNORECASE),
    ],
    "const_decl": [
        re.compile(r"^\+\s*(pub\s+)?const\s+[A-Z0-9_]+\s*[:=]", re.IGNORECASE),
    ],
    "dyn_trait": [
        re.compile(r"^\+.*\bdyn\s+[A-Z][A-Za-z0-9_]*", re.IGNORECASE),
        re.compile(r"^\+.*Box\s*<\s*dyn\s+[A-Z][A-Za-z0-9_]*", re.IGNORECASE),
    ],
    "hrtbs": [
        re.compile(r"^\+.*\bfor<'[a-z](?:,\s*'[a-z])*>\b", re.IGNORECASE),
        re.compile(r"^\+.*where\s+.*for<'[a-z]", re.IGNORECASE),
    ],
    "non_linear_ctrl_flow": [
        re.compile(r"^\+.*\b(match|loop|while|break|continue|return)\b"),
    ],
    "generics": [
        re.compile(r"^\+\s*(pub\s+)?(async\s+)?fn\s+\w+\s*<[^>]+>", re.IGNORECASE),
        re.compile(r"^\+.*\bimpl\s*<[^>]+>", re.IGNORECASE),
        re.compile(r"^\+.*\bwhere\b.*<[^>]+>", re.IGNORECASE),
        re.compile(r"^\+.*<[A-Za-z0-9_,\s:'?]+>"),
    ],
}

RUST_PATH_FILTER = ["*.rs"]

@dataclass
class CommitMeta:
    sha: str
    author_date: str
    author: str
    subject: str
    branch: str  # e.g., origin/main

def run(cmd: List[str], cwd: Path | None = None, check: bool = True) -> str:
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if check and res.returncode != 0:
        raise RuntimeError(f"Command failed ({' '.join(cmd)}):\n{res.stderr}")
    return res.stdout

def ensure_repo_cloned(root: Path, full_name: str, verbose: bool = False) -> Path:
    owner, name = full_name.split("/", 1)
    dest = root / name
    if dest.exists():
        if verbose:
            print(f"\033[1;34m[fetch]\033[0m {full_name}")
        run(["git", "fetch", "--all", "--tags", "--prune", "--quiet"], cwd=dest)
    else:
        if verbose:
            print(f"\033[1;34m[clone]\033[0m {full_name}")
        run(["git", "clone", "--filter=blob:none", f"https://github.com/{full_name}.git", str(dest)])
        run(["git", "fetch", "--all", "--tags", "--prune", "--quiet"], cwd=dest)
    return dest

def list_remote_branches(repo_dir: Path) -> List[str]:
    out = run(["git", "for-each-ref", "--format=%(refname:short)", "refs/remotes/origin"], cwd=repo_dir)
    return [b for b in (ln.strip() for ln in out.splitlines()) if b and b != "origin/HEAD"]

def branch_commit_count(repo_dir: Path, branch: str) -> int:
    try:
        out = run(["git", "rev-list", "--count", branch], cwd=repo_dir)
        return int(out.strip() or "0")
    except Exception:
        return 0

def pick_top_branches(repo_dir: Path, top_n: int = 2) -> List[str]:
    branches = list_remote_branches(repo_dir)
    scored = [(branch_commit_count(repo_dir, b), b) for b in branches]
    scored.sort(reverse=True)
    return [b for (cnt, b) in scored if cnt > 0][:top_n]

def build_log_args(branch: str, since: str | None, per_branch_limit: int) -> List[str]:
    args = ["git", "log", branch, "-i",
            "--pretty=format:%H|%ad|%an|%s", "--date=short", f"-n{per_branch_limit}"]
    for p in GREP_PATTERNS:
        args += ["--grep", p]
    if since:
        args += [f"--since={since}"]
    return args

def iter_branch_hits(repo_dir: Path, branch: str, since: str | None, per_branch_limit: int) -> Iterable[CommitMeta]:
    out = run(build_log_args(branch, since, per_branch_limit), cwd=repo_dir)
    for line in out.splitlines():
        if not line.strip():
            continue
        sha, ad, an, subject = line.split("|", 3)
        yield CommitMeta(sha=sha, author_date=ad, author=an, subject=subject, branch=branch)

def git_show_added_rust(repo_dir: Path, sha: str) -> str:
    cmd = ["git", "show", "--patch", "--find-renames", "--find-copies",
           "--unified=0", "--no-color", sha, "--"]
    cmd += RUST_PATH_FILTER
    return run(cmd, cwd=repo_dir)

def label_capabilities(diff_text: str) -> List[str]:
    hits: List[str] = []
    if not diff_text:
        return hits
    for cap, regexes in CAPABILITY_REGEXES.items():
        for rx in regexes:
            if rx.search(diff_text):
                hits.append(cap)
                break
    return hits

def parse_stat_line(repo_dir: Path, sha: str) -> Tuple[str, str, str]:
    stat = run(["git", "show", "--stat", "--oneline", "--no-color", sha], cwd=repo_dir).splitlines()
    tail = stat[-1] if stat else ""
    fc = re.search(r"(\d+)\s+files?\s+changed", tail)
    ins = re.search(r"(\d+)\s+insertions?\(\+\)", tail)
    dels = re.search(r"(\d+)\s+deletions?\(-\)", tail)
    return (
        fc.group(1) if fc else "",
        ins.group(1) if ins else "",
        dels.group(1) if dels else "",
    )

def short_sha(sha: str) -> str:
    return sha[:7]

def commit_url(full_name: str, sha: str) -> str:
    return f"https://github.com/{full_name}/commit/{sha}"

DEFAULT_PER_BRANCH_LIMIT = 200
BRANCHES = 3  # default number of branches to scan (top N by commit count)
OUT_PATH = "/home/matt/3712_Rust_Research_Project/evaluation/Results/extract_refs/extract_refs.csv"

def derive_unclassified_path(main_out: str) -> str:
    p = Path(main_out)
    stem = p.stem
    if p.suffix.lower() == ".csv":
        return str(p.with_name(f"{stem}_unclassified.csv"))
    return f"{main_out}_unclassified.csv"

def main():
    if Path(OUT_PATH).exists():
        idx = 1
        while True:
            new_path = Path(f"{OUT_PATH.rstrip('.csv')}_{idx}.csv")
            if not new_path.exists():
                Path(OUT_PATH).rename(new_path)
                print(f"Renamed existing {OUT_PATH} to {new_path}")
                break
            idx += 1

    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=OUT_PATH, help="Output CSV path for CLASSIFIED hits")
    ap.add_argument("--unclassified-out", default=None,
                    help="Output CSV path for UNCLASSIFIED hits (default: <out>_unclassified.csv)")
    ap.add_argument("--root", default="repos", help="Clone/fetch directory")
    ap.add_argument("--repo", action="append", help="Limit to specific repo(s); repeatable")
    ap.add_argument("--since", default=None, help="ISO date, e.g. 2019-01-01")
    ap.add_argument("--per-branch-limit", type=int, default=DEFAULT_PER_BRANCH_LIMIT,
                    help="Max commits per selected branch (0 = no limit)")
    ap.add_argument("--top-branches", type=int, default=BRANCHES,
                    help="Number of remote branches to scan, sorted by total commits")
    ap.add_argument("--verbose", action="store_true")
    args = ap.parse_args()

    repos = args.repo if args.repo else DEFAULT_REPOS
    root = Path(args.root).resolve()
    root.mkdir(parents=True, exist_ok=True)

    unclassified_out = args.unclassified_out or derive_unclassified_path(args.out)

    # Writers for classified and unclassified
    classified_file = open(args.out, "w", newline="", encoding="utf-8")
    unclassified_file = open(unclassified_out, "w", newline="", encoding="utf-8")

    classified_writer = csv.writer(classified_file)
    unclassified_writer = csv.writer(unclassified_file)

    headers = [
        "repo", "commit", "commit_url", "author_date", "author", "subject",
        "hit_aspects", "files_changed", "insertions", "deletions", "branch"
    ]
    classified_writer.writerow(headers)
    unclassified_writer.writerow(headers)

    try:
        for full_name in repos:
            try:
                repo_dir = ensure_repo_cloned(root, full_name, verbose=args.verbose)
            except Exception as e:
                print(f"[error] clone/fetch {full_name}: {e}", file=sys.stderr)
                continue

            print(f"\n\033[1;33m>>> Scanning {full_name}\033[0m")
            try:
                branches = pick_top_branches(repo_dir, top_n=args.top_branches)
                if not branches:
                    print(f"[warn] no remote branches found for {full_name}")
                    continue

                seen: Set[str] = set()  # de-dup across branches
                for br in branches:
                    for meta in iter_branch_hits(repo_dir, br, args.since, args.per_branch_limit):
                        if meta.sha in seen:
                            continue
                        seen.add(meta.sha)

                        diff = git_show_added_rust(repo_dir, meta.sha)
                        added_lines = "\n".join(
                            ln for ln in diff.splitlines() if ln.startswith("+") and not ln.startswith("+++")
                        )
                        aspects = label_capabilities(added_lines)

                        fc, ins, dels = parse_stat_line(repo_dir, meta.sha)
                        url = commit_url(full_name, meta.sha)

                        repo_color = "\033[96m"
                        reset = "\033[0m"
                        sha_short = short_sha(meta.sha)

                        if aspects:
                            classified_writer.writerow([
                                full_name, meta.sha, url, meta.author_date, meta.author,
                                meta.subject, " ".join(aspects), fc, ins, dels, br
                            ])
                            tag_color = "\033[92m"  # green
                            aspects_str = ", ".join(aspects)
                            print(f"{repo_color}{full_name:<25}{reset} "
                                  f"{meta.author_date} {sha_short} "
                                  f"{tag_color}[{aspects_str}]{reset} {meta.subject}\n"
                                  f"    \033[94m{url}\033[0m  ({br})")
                        else:
                            # Store & print UNCLASSIFIED hits
                            unclassified_writer.writerow([
                                full_name, meta.sha, url, meta.author_date, meta.author,
                                meta.subject, "", fc, ins, dels, br
                            ])
                            tag_color = "\033[93m"  # yellow
                            print(f"{repo_color}{full_name:<25}{reset} "
                                  f"{meta.author_date} {sha_short} "
                                  f"{tag_color}[unclassified]{reset} {meta.subject}\n"
                                  f"    \033[94m{url}\033[0m  ({br})")

            except Exception as e:
                print(f"[error] scanning {full_name}: {e}", file=sys.stderr)

    finally:
        classified_file.close()
        unclassified_file.close()

    print(f"\nWrote CLASSIFIED CSV to {args.out}")
    print(f"Wrote UNCLASSIFIED CSV to {unclassified_out}")

if __name__ == "__main__":
    main()
