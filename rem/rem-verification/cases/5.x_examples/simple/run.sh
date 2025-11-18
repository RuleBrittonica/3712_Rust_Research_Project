#!/bin/bash
set -e  # stop if any command fails

/bin/python3 /home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple/llbc.py
/bin/python3 /home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple/coq.py
/bin/python3 /home/matt/3712_Rust_Research_Project/rem/rem-verification/cases/5.x_examples/simple/equiv.py
