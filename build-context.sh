#!/bin/bash

# Usage: ./build.sh <directory> [-e ext1] [-e ext2] ...
# Example: ./build.sh ./src -e lua -e fs

if [ -z "$1" ]; then
    echo "Usage: $0 <directory> [-e extension]"
    exit 1
fi

target_dir="$1"
shift # Remove the directory from the argument list so we can process flags

# Initialize variables
extensions=()
context_file="context.txt"
tree_file="tree.tmp"
context_body="body.tmp"

# Loop through remaining arguments to find extensions
while [[ $# -gt 0 ]]; do
    case "$1" in
        -e|--extension)
            if [[ -n "$2" && "$2" != -* ]]; then
                extensions+=("-e" "$2")
                shift 2
            else
                echo "Error: -e requires an extension"
                exit 1
            fi
            ;;
        *)
            # Ignore other arguments or handle them here
            shift
            ;;
    esac
done

# If no extensions were provided, default to txt
if [ ${#extensions[@]} -eq 0 ]; then
    extensions+=("-e" "txt")
fi

echo "Building context for: ${target_dir}..."

# 1. Build the tree structure
printf "<begin tree>\n" > "${tree_file}"
if command -v eza &> /dev/null; then
    eza --tree "${target_dir}" >> "${tree_file}"
else
    tree "${target_dir}" >> "${tree_file}"
fi
printf "<end tree>\n\n" >> "${tree_file}"

# 2. Run aggregate using the array of extensions
if [ -f "./aggregate" ]; then
    # "${extensions[@]}" expands the array into separate -e ext arguments
    ./aggregate -d "${target_dir}" "${extensions[@]}" -o "${context_body}"
else
    echo "Error: ./aggregate tool not found."
    exit 1
fi

# 3. Combine into final file
cat "${tree_file}" "${context_body}" > "${context_file}"

# 4. Cleanup
rm "${tree_file}" "${context_body}"

echo "Finished building ${context_file}"
