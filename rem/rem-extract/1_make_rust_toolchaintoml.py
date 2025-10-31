# Makes a file called rust-toolchain.toml
# The file contains the following:
# [toolchain]
# channel = "nightly-2024-08-28"
# components = [
#     "rustc-src",
#     "rustc-dev",
#     "llvm-tools-preview",
#     "rust-analyzer-preview",
#     "rustfmt",
# ]

# profile = "minimal"

import os

toml_path = os.path.join(os.getcwd(), 'rust-toolchain.toml')

# Delete the file if it already exists
if os.path.exists(toml_path):
    os.remove(toml_path)

with open(toml_path, 'w') as toml_file:
    toml_file.write('[toolchain]\n')
    toml_file.write('channel = "nightly-2024-08-28"\n')
    toml_file.write('components = [\n')
    toml_file.write('    "rustc-src",\n')
    toml_file.write('    "rustc-dev",\n')
    toml_file.write('    "llvm-tools-preview",\n')
    toml_file.write('    "rust-analyzer-preview",\n')
    toml_file.write('    "rustfmt",\n')
    toml_file.write(']\n')
    toml_file.write('\n')
    toml_file.write('profile = "minimal"')