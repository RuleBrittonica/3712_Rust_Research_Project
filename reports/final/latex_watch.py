import sys
import time
import subprocess
import os
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

MAIN_TEX_FILE = "main.tex"

class LatexCompileHandler(FileSystemEventHandler):
    def __init__(self):
        super().__init__()
        self.is_compiling = False

    def on_any_event(self, event):
        """
        This method is triggered on any file system event: creation, modification, deletion, etc.
        We'll invoke latexmk here if we're not already in the middle of a compile.
        """
        if self.is_compiling:
            # Prevent concurrency (i.e., if multiple events come in at once).
            return

        self.is_compiling = True
        try:
            compile_latex()
        finally:
            self.is_compiling = False

def compile_latex():
    """
    Run latexmk with:
    - -pdf        => compile to PDF
    - -shell-escape => needed for minted
    """
    print("Detected file change. Compiling LaTeX...\n")
    try:
        subprocess.run(
            ["latexmk", "-pdf", "-shell-escape", MAIN_TEX_FILE],
            check=True
        )
        print("Compilation successful.\n")
    except subprocess.CalledProcessError:
        print("Compilation failed. Check LaTeX errors.\n")

def main():
    if not os.path.isfile(MAIN_TEX_FILE):
        print(f"Error: '{MAIN_TEX_FILE}' not found in the current directory.")
        sys.exit(1)

    event_handler = LatexCompileHandler()
    observer = Observer()
    observer.schedule(event_handler, path=".", recursive=True)
    observer.start()

    print(f"Watching for changes in '{os.getcwd()}' and subfolders...\n")
    print(f"Will automatically compile '{MAIN_TEX_FILE}' on changes.\n")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()

if __name__ == "__main__":
    main()
