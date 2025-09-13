from pptx import Presentation
from pptx.util import Mm, Pt
from pptx.enum.text import PP_ALIGN
import os
import psutil
import win32com.client as win32
import subprocess

OUTPUT_PPTX = "REMV_SPLASH_POSTER.pptx"
OUTPUT_PDF  = "REMV_SPLASH_POSTER.pdf"

def close_powerpoint():
    """Kill PowerPoint if running."""
    for proc in psutil.process_iter(['pid', 'name']):
        if proc.info['name'] and "POWERPNT.EXE" in proc.info['name']:
            print("Closing PowerPoint...")
            proc.kill()

def export_pdf(pptx_path, pdf_path):
    """Export a PPTX file to PDF using PowerPoint COM automation."""
    ppt_app = win32.Dispatch("PowerPoint.Application")
    ppt_app.Visible = 1
    presentation = ppt_app.Presentations.Open(os.path.abspath(pptx_path))
    presentation.SaveAs(os.path.abspath(pdf_path), 32)  # 32 = PDF format
    presentation.Close()
    ppt_app.Quit()
    print(f"Exported {pdf_path}")

def open_powerpoint(pptx_path):
    """Open file in PowerPoint."""
    subprocess.Popen(["powerpnt.exe", os.path.abspath(pptx_path)])
    print("Reopened PowerPoint.")

prs = Presentation()

# Define custom slide size: 1000 mm x 1000 mm (1m x 1m)
prs.slide_width = Mm(1000)
prs.slide_height = Mm(1000)

# Add a blank slide
slide_layout = prs.slide_layouts[6]
slide = prs.slides.add_slide(slide_layout)

def add_textbox(slide, left, top, width, height, text, font_size=32, bold=False, align=PP_ALIGN.LEFT):
    txBox = slide.shapes.add_textbox(Mm(left), Mm(top), Mm(width), Mm(height))
    tf = txBox.text_frame
    tf.clear()
    p = tf.add_paragraph()
    p.text = text
    p.font.size = Pt(font_size)
    p.font.bold = bold
    p.alignment = align
    return txBox

def add_image(slide, left, top, width, height, path="poster\\images\\sample.png", caption=None, caption_size=30):
    """Add an image with an optional caption below it."""
    pic = slide.shapes.add_picture(path, Mm(left), Mm(top), Mm(width), Mm(height))
    if caption:
        add_textbox(
            slide,
            left,
            top + height + 5,   # position just below image (5 mm gap)
            width,
            10,                 # caption box height
            caption,
            font_size=caption_size,
            align=PP_ALIGN.CENTER
        )
    return pic

# Title banner
add_textbox(
    slide,
    20,
    10,
    960,
    70,
    "Verifying Extract Method Refactoring in Rust",
    font_size=100,
    bold=True,
    align=PP_ALIGN.CENTER,
)

add_textbox(
    slide,
    20,
    90,
    960,
    40,
    "Matthew Britton, Alex Potanin, Sasha Pak\nAustralian National University",
    font_size=48,
)

# Logos
add_image(slide, 20, 10, 60, 60)    # ANU logo
add_image(slide, 920, 10, 60, 60)   # SPLASH logo

# Column setup
col_width = 1000 / 3
margin = 20

# Left column: Motivation
add_textbox(
    slide,
    margin,
    150,
    col_width - 2 * margin,
    800,
    "Motivation & Problem\n\n- Rust refactoring is hard (ownership, lifetimes, effects)\n"
    "- Compilation success ≠ semantic equivalence\n"
    "- Need trustworthy, automated refactoring",
    font_size=36,
)

# Middle column: Pipeline
add_textbox(
    slide,
    col_width + margin,
    150,
    col_width - 2 * margin,
    800,
    "Approach & Pipeline\n\n[Diagram Placeholder]\n\nREM → CHARON → Aeneas → Coq\n"
    "Zero-annotation, IDE integration",
    font_size=36,
)

# Right column: Results + Expansion
add_textbox(
    slide,
    2 * col_width + margin,
    150,
    col_width - 2 * margin,
    800,
    "Results & Expansion\n\n- 10/10 cases discharged\n- Avg cycle: 2s (IDE-friendly)\n\n"
    "From Prototype to Production-Ready REM:\n"
    "- Standalone CLI\n- Async/await, generics, macros\n- VSCode extension\n\n"
    "Future Work:\n- Unsafe code\n- Concurrency\n- Large-scale evaluation",
    font_size=36,
)

add_image(slide, 900, 880, 80, 80, caption="GitHub")
add_image(slide, 800, 880, 80, 80, caption="VSCode")
add_image(slide, 700, 880, 80, 80, caption="DOI")

prs.save("REMV_SPLASH_POSTER.pptx")
print("Saved Poster")


def main():
    close_powerpoint()
    export_pdf(OUTPUT_PPTX, OUTPUT_PDF)

if __name__ == "__main__":
    main()