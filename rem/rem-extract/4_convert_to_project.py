import os
import subprocess

# Directory containing the input Rust files
input_directory = 'input'

# Check if the input directory exists
if not os.path.exists(input_directory):
    print(f"The directory '{input_directory}' does not exist.")
    exit(1)

# Read in rust-toolchain.toml
# Copy the contents of the file to a string
# Delete the file (temporarily)
toolchain = 'rust-toolchain.toml'
with open(toolchain, 'r') as f:
    toolchain_contents = f.read()
    f.close()
import os
os.remove(toolchain)

# Iterate over each file in the input directory
for filename in os.listdir(input_directory):
    if filename.endswith('.rs'):
        # Extract the base name without extension
        project_name = os.path.splitext(filename)[0]

        # Define the path for the new project
        project_path = os.path.join(input_directory, project_name)

        # Create a new Rust project
        result = subprocess.run(
            ['cargo', 'new', '--bin', project_name],
            cwd=input_directory,
            capture_output=True,
            text=True,
            env=dict(CARGO_TERM_COLOR='always')  # Ensure Cargo uses color
        )

        # Filter out te see more line
        filtered_stdout = "\n".join(
            line for line in result.stdout.splitlines()
            if "see more `Cargo.toml` keys and their definitions" not in line
        )
        print(filtered_stdout)

        filtered_stderr = "\n".join(
            line for line in result.stderr.splitlines()
            if "see more `Cargo.toml` keys and their definitions" not in line
        )
        print(filtered_stderr)

        # Define the path for the main.rs file in the new project
        main_rs_path = os.path.join(project_path, 'src', 'main.rs')

        # Define the path for the original .rs file
        original_rs_path = os.path.join(input_directory, filename)

        # Copy the contents of the original .rs file to main.rs
        with open(original_rs_path, 'r') as original_file:
            code = original_file.read()

        # Write the original code to main.rs
        with open(main_rs_path, 'w') as main_file:
            main_file.write(code)

        # Clear the contents of main.rs to remove the default `fn main`
        with open(main_rs_path, 'w') as main_file:
            main_file.write("")  # Clear the file

        # Write the original code to main.rs again
        with open(main_rs_path, 'w') as main_file:
            main_file.write(code)

        # Delete the original .rs file
        os.remove(original_rs_path)

        # Color the output - project name in green, input directory in blue,
        # deleted file in orange
        print(f"\tProject \033[92m{project_name}\033[0m created in \033[94m{input_directory}\033[0m, \033[93m{filename}\033[0m deleted")
# For every project in the input directory, go to the main.rs file and add a
# main function at the end of the file (this main function does nothing, but it
# satisfies the Rust compiler)
for folder in os.listdir(input_directory):
    if os.path.isdir(os.path.join(input_directory, folder)):
        main_rs_path = os.path.join(input_directory, folder, 'src', 'main.rs')
        with open(main_rs_path, 'a') as main_file:
            main_file.write("\n\nfn main() {\n\n}\n")

# Define the directory containing the files
directory = "correct_output"
# Loop through all files in the directory
for filename in os.listdir(directory):
    file_path = os.path.join(directory, filename)
    if os.path.isfile(file_path):
        with open(file_path, "a") as file:
            file.write("\n\nfn main() {\n\n}\n")

# Create the toolchain file
with open(toolchain, 'w') as f:
    f.write(toolchain_contents)
    f.close()