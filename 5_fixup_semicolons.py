# Goes through all .rs files in input/ and correct_output/ and  removes any
# replaces any instance of "};" with "}"

# For the input dir, we have to go through each of the projects (src/main.rs),
# For the correct_output dir, we have to go through each of the files

import os

input_directory = "input"
correct_output_directory = "correct_output"

acceptable_files = [
    "method_with_reference",
    "method_with_mut",
    "struct_with_two_fields_pattern_define_variables",
]

def is_acceptable(name):
    return any(acceptable in name for acceptable in acceptable_files)

# Process files in the correct_output directory
for rs_file in os.listdir(correct_output_directory):
    if rs_file.endswith('.rs') and is_acceptable(rs_file):
        rs_path = os.path.join(correct_output_directory, rs_file)
        with open(rs_path, 'r') as file:
            code = file.read()
        code = code.replace("};", "}")
        with open(rs_path, 'w') as file:
            file.write(code)

# Process projects in the input directory
for project in os.listdir(input_directory):
    if is_acceptable(project):
        rs_path = os.path.join(input_directory, project, 'src', 'main.rs')
        if os.path.exists(rs_path):  # Check if main.rs exists
            with open(rs_path, 'r') as file:
                code = file.read()
            code = code.replace("};", "}")
            with open(rs_path, 'w') as file:
                file.write(code)

print("Completed processing acceptable files.")