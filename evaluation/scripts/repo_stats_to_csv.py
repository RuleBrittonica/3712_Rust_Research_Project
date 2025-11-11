import requests
import csv

repos = [
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
    "typst/typst"
]

output_file = "/home/matt/3712_Rust_Research_Project/evaluation/Results/repo_stats.csv"

API_URL = "https://api.github.com/repos/{}"

TOKEN = None
headers = {"Authorization": f"token {TOKEN}"} if TOKEN else {}

rows = []
for repo in repos:
    url = API_URL.format(repo)
    print(f"Fetching {repo} ...")
    r = requests.get(url, headers=headers)
    if r.status_code == 200:
        data = r.json()
        rows.append({
            "full_name": data["full_name"],
            "description": data.get("description", ""),
            "language": data.get("language", ""),
            "stargazers_count": data.get("stargazers_count", 0),
            "forks_count": data.get("forks_count", 0),
            "open_issues_count": data.get("open_issues_count", 0),
            "watchers_count": data.get("watchers_count", 0),
            "created_at": data.get("created_at", ""),
            "updated_at": data.get("updated_at", ""),
            "html_url": data.get("html_url", "")
        })
    else:
        print(f"Failed to fetch {repo}: {r.status_code}")

with open(output_file, "w", newline="", encoding="utf-8") as f:
    writer = csv.DictWriter(f, fieldnames=rows[0].keys())
    writer.writeheader()
    writer.writerows(rows)

print(f"\nSaved data for {len(rows)} repositories to {output_file}")