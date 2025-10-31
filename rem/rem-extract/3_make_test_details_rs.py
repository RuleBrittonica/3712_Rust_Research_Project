import csv
import os

# Input CSV file containing the test details
csv_file = '0_test_info.csv'

# Output Rust file
rust_file = 'rem-extract/src/test_details.rs'

# Make the rust file if it doesn't exist
if not os.path.exists(rust_file):
    with open(rust_file, 'w') as file:
        file.write("")

# Function to create the Rust file from the CSV data
def create_rust_file(csv_file, rust_file):
    with open(csv_file, 'r') as csvfile, open(rust_file, 'w') as rustfile:
        csv_reader = csv.DictReader(csvfile)

        rustfile.write("use lazy_static::lazy_static;\n")
        rustfile.write("use crate::extract_tests::TestFile;\n")
        rustfile.write("lazy_static! {\n")
        rustfile.write("    pub static ref TEST_FILES: Vec<TestFile<'static>> = vec![\n")

        for row in csv_reader:
            test_name = row['Test name']
            input_cursor_1 = row['Input cursor 1 location']
            input_cursor_2 = row['Input cursor 2 location']

            # Parse line numbers from the cursor positions
            rustfile.write(f'        TestFile::new(\n')
            rustfile.write(f'            "{test_name}",\n')
            rustfile.write(f'            {input_cursor_1},\n')
            rustfile.write(f'            {input_cursor_2},\n')
            rustfile.write(f'        ),\n')

        rustfile.write("    ];\n")
        rustfile.write("}\n")

# Run the script
create_rust_file(csv_file, rust_file)
print(f"Rust file '{rust_file}' has been created.")
