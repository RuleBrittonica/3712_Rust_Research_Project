# Replace the members = [] section of the Cargo.toml file with just members =
# ["rem-extract"].

toml_file_path = "Cargo.toml"

def fixup_cargo_toml():
    try:
        with open(toml_file_path, 'r', encoding='utf-8') as file:
            content = file.read()

            # Find the start and end indices of the members list
            start_index = content.find("members = [")
            end_index = content.find("]", start_index) + 1

            # Replace the members list with just the rem-extract string
            new_content = content[:start_index] + 'members = [\n"rem-extract"\n]' + content[end_index:]

        with open(toml_file_path, 'w', encoding='utf-8') as file:
            file.write(new_content)

        print(f"Successfully fixed up '{toml_file_path}'.")

    except FileNotFoundError:
        print(f"Error: File '{toml_file_path}' not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

def main():
    fixup_cargo_toml()

if __name__ == "__main__":
    main()