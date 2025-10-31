import sys

def calculate_offset(file_path, target_string, occurrence=1):
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()

            current_offset = 0
            found_occurrences = 0

            while True:
                # Find the next occurrence of the target string starting from the current offset
                current_offset = content.find(target_string, current_offset)

                if current_offset == -1:
                    print(f"String '{target_string}' not found at occurrence {occurrence}.")
                    return

                # Increment the occurrence count
                found_occurrences += 1

                if found_occurrences == occurrence:
                    print(f"The offset of occurrence {occurrence} of '{target_string}' is: {current_offset}")
                    return

                # Move to the next character after the found string for further searching
                current_offset += len(target_string)

    except FileNotFoundError:
        print(f"Error: File '{file_path}' not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

def main():
    if len(sys.argv) < 3 or len(sys.argv) > 4:
        print("Usage: python script.py <file_path> <string_to_find> [occurrence]")
    else:
        file_path = sys.argv[1]
        target_string = sys.argv[2]
        occurrence = int(sys.argv[3]) if len(sys.argv) == 4 else 1
        calculate_offset(file_path, target_string, occurrence)

if __name__ == "__main__":
    main()