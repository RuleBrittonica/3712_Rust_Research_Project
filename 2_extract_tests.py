import os
import re
import csv

# Print these out to the console, using red text and a bold font
if os.name == 'nt':
    print("\033[1;31;40mDetected running on windows. Assuming CRLF line endings.\033[0m")
    LINE_END = 2
else:
    print("\033[1;31;40mDetected running on unix. Assuming LF line endings.\033[0m")
    LINE_END = 1

# Input test file containing the unit tests
input_file = '0_extract_tests.rs'  # Replace with your actual input file

# Output directories for input and output .rs files
input_dir = 'input'
output_dir = 'correct_output'
os.makedirs(input_dir, exist_ok=True)
os.makedirs(output_dir, exist_ok=True)

# CSV file to store the cursor positions
csv_file = '0_test_info.csv'

# Regex patterns for extracting the test name and raw strings
test_name_pattern = re.compile(r'fn (\w+)\(')
raw_string_pattern = re.compile(r'r#"(.*?)"#', re.DOTALL)

# Function to find cursor positions ($0) in the code
def find_cursor_position(code):
    cursor = '$'
    counter = 0
    index = 0
    # Split the file by newlines
    lines = code.split('\n')
    # if the line doesn't contain a $ symbol, add its length to the counter
    for idx, line in enumerate(lines):
        # break out if the line has a $ sign in it
        if cursor in line:
            index = idx
            break
        counter += len(line)
        counter += LINE_END # Account for the CR + LF?

    final_line = lines[index]
    # Split the final line by the $, count the part up to it
    parts = final_line.split(cursor)
    counter += len(parts[0])

    return counter

affected_files = []

# Returns the distance between the end of the first cursor and the start of the
# second cursor
# E.g., for the code:
# fn foo() {
#     foo($01 + 1$0);
# }
# We would return 5 (len(" + 1") = 5)
# This method also needs to be aware of the CR + LF line endings
def distance_between_cursors(code):
    counter = 0
    # Split the code by the $0
    cursor = '$0'
    new_code = code.split(cursor)[1]
    # Split the code by the \n
    lines = new_code.split('\n')
    for line in lines:
        # If the line contains a $ sign, break out
        counter += len(line)
        counter += LINE_END
    return counter - LINE_END # Don't add CRLF for the last line


# Function to remove comments from the first line of the code
# E.g., for the code:
# // This is a comment
# fn foo() {
#     println!("Hello, world!");
# }
# The function would return:
# fn foo() {
#     println!("Hello, world!");
# }
def remove_first_line_comment(code):
    lines = code.split('\n')
    if len(lines) > 0:
        lines[0] = re.sub(r'//.*', '', lines[0])
    new_lines = '\n'.join(lines)
    if code != new_lines:
        affected_files.append(code)
    return new_lines

# Function to replace references to core:: with std:: in the code
def replace_core_with_std(code):
    new_code = code.replace('core::', 'std::')
    if code != new_code:
        affected_files.append(code)
    return new_code

def remove_double_semicolons(code):
    new_code = code.replace(';;', ';')
    if code != new_code:
        affected_files.append(code)
    return new_code

# Function to process each test and generate files and CSV entries
def process_tests(input_file, csv_file):
    with open(input_file, 'r') as file, open(csv_file, 'w', newline='') as csvfile:
        csv_writer = csv.writer(csvfile)
        csv_writer.writerow(['Test name', 'Input cursor 1 location', 'Input cursor 2 location'])

        content = file.read()
        test_blocks = content.split('#[test]')

        for test_block in test_blocks[1:]:  # Skip the first element since it's before the first test

            test_name_match = test_name_pattern.search(test_block)
            raw_strings = raw_string_pattern.findall(test_block)


            if test_name_match and len(raw_strings) == 2:
                test_name = test_name_match.group(1)
                input_code = raw_strings[0].strip()  # Remove leading and trailing whitespace
                output_code = raw_strings[1].strip()  # Remove leading and trailing whitespace

                # For the input and output code, remove comments from the first
                # line
                input_code = remove_first_line_comment(input_code)
                output_code = remove_first_line_comment(output_code)

                input_code = replace_core_with_std(input_code)
                output_code = replace_core_with_std(output_code)

                input_code = remove_double_semicolons(input_code)
                output_code = remove_double_semicolons(output_code)

                input_code = input_code.strip()
                output_code = output_code.strip()

                # Find cursor positions in the input code
                input_cursor_1 = find_cursor_position(input_code)
                distance_between_cursors_ = distance_between_cursors(input_code)
                input_cursor_2 = input_cursor_1 + distance_between_cursors_

                print(f"Processing test: {test_name}, Input cursor 1: {input_cursor_1}, Input cursor 2: {input_cursor_2}")


                input_code_cleaned = input_code.replace('$0', '')
                output_code_cleaned = output_code.replace('$0', '')

                with open(os.path.join(input_dir, f'{test_name}.rs'), 'w') as infile:
                    infile.write(input_code_cleaned)
                with open(os.path.join(output_dir, f'{test_name}.rs'), 'w') as outfile:
                    outfile.write(output_code_cleaned)

                # Write character-based cursor information to CSV
                csv_writer.writerow([test_name, input_cursor_1, input_cursor_2])


# Run the script
process_tests(input_file, csv_file)
print(f"Processing complete. Files are saved in '{input_dir}' and '{output_dir}', and the CSV file is '{csv_file}'.")

# Open up the CSV file, and sort the rows alphabetically by the test name
import pandas as pd
df = pd.read_csv(csv_file)
df = df.sort_values(by='Test name')
df.to_csv(csv_file, index=False)
print(f"CSV file sorted alphabetically by test name.")

print("Made alterations to the following files:")
for file in affected_files:
    print(f"\t- {file}")
