# Coverage + Performance charts for Slide 1
import matplotlib.pyplot as plt

# --- Coverage ---
total_cases = 40
success, pending = 26, 4
failed = total_cases - success - pending
labels = [f"Successful ({success})", f"Pending – minor fixes ({pending})", f"Unsupported/Failed ({failed})"]
sizes = [success, pending, failed]

fig1, ax1 = plt.subplots(figsize=(6, 6))
ax1.pie(
    sizes, explode=(0.05, 0, 0), labels=labels,
    autopct=lambda p: f"{p:.1f}%", startangle=90, counterclock=False
)
ax1.set_title("Extraction Coverage (40 new real-world cases)", pad=16)
ax1.axis('equal')
plt.tight_layout()
plt.savefig("extraction_coverage_pie.png", dpi=200, transparent=True)

# --- Performance ---
labels = ["REM (IntelliJ)", "New Toolchain (VSCode end-to-end)", "Query Only (IR cache hit)"]
times  = [0.949, 0.47, 0.01]

fig2, ax2 = plt.subplots(figsize=(7, 5))
bars = ax2.bar(labels, times)
ax2.set_ylabel("Average time (seconds)")
ax2.set_title("Extraction Performance")
ax2.set_ylim(0, max(times)*1.25)
for bar, t in zip(bars, times):
    ax2.text(bar.get_x()+bar.get_width()/2, bar.get_height()+max(times)*0.03, f"{t:.3f}s", ha="center", va="bottom")
plt.xticks(rotation=12, ha="right")
plt.tight_layout()
plt.savefig("extraction_performance_bar.png", dpi=200, transparent=True)
