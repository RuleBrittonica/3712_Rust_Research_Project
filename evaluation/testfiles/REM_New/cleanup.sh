#!/usr/bin/env bash
# Removes non-code files larger than 100KB from the dir it is run in.
# Useful for cleaning up after running a bunch of extract with repair / verify
# as we generate a lot of large binary files that are not needed.

TARGET_DIR="${1:-.}"

SIZE="+50k"

CODE_EXTENSIONS="c|cpp|h|hpp|py|js|ts|java|go|rb|php|sh|rs|swift|html|css|xml|json|yaml|yml|md|txt|toml|ini"

echo "Scanning: $TARGET_DIR"
echo "Deleting non-code files larger than 100 KB..."
echo

# Find files >100KB that do NOT match allowed code extensions
find "$TARGET_DIR" -type f -size "$SIZE" \
    | grep -Ev "\.($CODE_EXTENSIONS)$" \
    | while read -r file; do
        echo "Deleting: $file"
        rm -f "$file"
    done

echo
echo "Done."