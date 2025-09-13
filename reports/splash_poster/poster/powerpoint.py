from pptx import Presentation
from pptx.util import Inches, Pt

# --- Setup presentation ---
prs = Presentation()

# Define custom slide size: 100 cm x 100 cm (~39.37 inches)
prs.slide_width = Inches(39.37)
prs.slide_height = Inches(39.37)

# Add a blank slide
slide_layout = prs.slide_layouts[6]  # blank layout
slide = prs.slides.add_slide(slide_layout)

# --- Helpers ---
def add_textbox(slide, left, top, width, height, text, font_size=32, bold=False):
    txBox = slide.shapes.add_textbox(left, top, width, height)
    tf = txBox.text_frame
    p = tf.add_paragraph()
    p.text = text
    p.font.size = Pt(font_size)
    p.font.bold = bold
    return txBox

# --- Title banner ---
add_textbox(
    slide,
    Inches(0.5),
    Inches(0.2),
    Inches(38),
    Inches(2),
    "Verifying Extract Method Refactoring in Rust",
    font_size=100,
    bold=True,
)

add_textbox(
    slide,
    Inches(0.5),
    Inches(2.5),
    Inches(38),
    Inches(1.0),
    "Matthew Britton, Alex Potanin, Sasha Pak\nAustralian National University",
    font_size=48,
)

# --- Column guides ---
col_width = prs.slide_width / 3
margin = Inches(0.5)

# Left column: Motivation
add_textbox(
    slide,
    margin,
    Inches(4),
    col_width - Inches(1),
    prs.slide_height - Inches(5),
    "Motivation & Problem\n\n- Rust refactoring is hard (ownership, lifetimes, effects)\n- Compilation success ≠ semantic equivalence\n- Need trustworthy, automated refactoring",
    font_size=36,
)

# Middle column: Pipeline
add_textbox(
    slide,
    col_width + margin,
    Inches(4),
    col_width - Inches(1),
    prs.slide_height - Inches(5),
    "Approach & Pipeline\n\n[Diagram Placeholder]\n\nREM → CHARON → Aeneas → Coq\n\nZero-annotation, IDE integration",
    font_size=36,
)

# Right column: Results + Future Work
add_textbox(
    slide,
    col_width * 2 + margin,
    Inches(4),
    col_width - Inches(1),
    prs.slide_height - Inches(5),
    "Results & Expansion\n\n- 10/10 cases discharged\n- Avg cycle: 2s (IDE-friendly)\n\nFrom Prototype to Production-Ready REM:\n- Standalone CLI\n- Async/await, generics, macros\n- VSCode extension\n\nFuture Work:\n- Unsafe code\n- Concurrency\n- Large-scale evaluation",
    font_size=36,
)

# --- Save file ---
prs.save("SPLASH25_Poster_Skeleton.pptx")
print("Poster skeleton saved as SPLASH25_Poster_Skeleton.pptx")
