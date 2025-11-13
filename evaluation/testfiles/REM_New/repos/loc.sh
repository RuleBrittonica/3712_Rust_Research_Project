for repo in $(find . -maxdepth 1 -mindepth 1 -type d | sort); do
    name=$(basename "$repo")
    echo "=== $name ==="

    find "$repo" -type f -name "*.rs" \
      | xargs wc -l 2>/dev/null \
      | tail -n 1

    echo
done