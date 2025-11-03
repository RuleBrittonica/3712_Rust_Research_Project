import pandas as pd
import matplotlib.pyplot as plt
import os

csv_path = "/home/matt/3712_Rust_Research_Project/evaluation/Results/rem_extract_metrics_processed.csv"
df = pd.read_csv(csv_path)

# Columns to always remove
drop_cols = [c for c in ["total_time", "total_time_as_ms"] if c in df.columns]
df = df.drop(columns=drop_cols, errors="ignore")

# Detect REM_Original path column
path_col = None
for col in df.columns:
    if df[col].astype(str).str.contains("REM_Original", na=False).any():
        path_col = col
        break

if path_col is None:
    raise ValueError("No path column containing 'REM_Original' found!")

def extract_case(path):
    if isinstance(path, str) and "/REM_Original/" in path:
        return path.split("/REM_Original/")[1].split("/")[0]
    return "unknown"

df["case"] = df[path_col].map(extract_case)

# Identify numeric stage timing columns
time_cols = [
    c for c in df.columns
    if c not in [path_col, "case"] and pd.to_numeric(df[c], errors="coerce").notna().any()
]

# Stage name cleaning
def clean_stage(stage):
    if "__to__" in stage:
        stage = stage.split("__to__")[-1]
    stage = stage.replace("extraction_", "")
    stage = stage.replace("_", " ").strip().title()
    return stage

# Convert to long form
long = df.melt(id_vars=["case"], value_vars=time_cols,
               var_name="stage", value_name="time_ns")

long["time_ns"] = pd.to_numeric(long["time_ns"], errors="coerce")
long = long.dropna(subset=["time_ns"])

# Convert to seconds
long["time_sec"] = long["time_ns"] / 1e9

long["stage_clean"] = long["stage"].map(clean_stage)

# Aggregate
agg = long.groupby(["case", "stage_clean"], as_index=False)["time_sec"].sum()

# Wide form for plotting
wide = agg.pivot(index="case", columns="stage_clean",
                 values="time_sec").fillna(0)

# Sort by total duration ascending
wide["__total__"] = wide.sum(axis=1)
wide = wide.sort_values("__total__", ascending=True).drop(columns="__total__")

# Order stacked segments by average contribution
stage_order = wide.mean().sort_values(ascending=False).index.tolist()
wide = wide[stage_order]

# Plot
plt.figure(figsize=(14, 10))
left = None
for stage in stage_order:
    vals = wide[stage].values
    if left is None:
        plt.barh(wide.index, vals, label=stage)
        left = vals
    else:
        plt.barh(wide.index, vals, left=left, label=stage)
        left = left + vals

plt.xlabel("Time (seconds)")
plt.ylabel("Test Case")
plt.title("REM Extraction Time Breakdown by Stage")
plt.xticks(rotation=0)
plt.legend(
    title="Stage",
    loc="lower right",
    bbox_to_anchor=(1.0, 0.02),
    frameon=True,
    edgecolor="black"
)

plt.tight_layout()

plt.tight_layout()

# Save figure
out_path = os.path.join(os.path.dirname(csv_path), "rem_extraction_time_breakdown.png")
plt.savefig(out_path, dpi=300)
print("Saved figure:", out_path)
