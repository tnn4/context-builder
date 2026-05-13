#!/bin/bash

# Usage: ./build.sh <directory> [-e ext1] [-e ext2] ... [-o output_filename]
# Example: ./build.sh ./src -e lua -e fs -o my_context.txt

if [ -z "$1" ]; then
    echo "Usage: $0 <directory> [-e extension] [-o output_file]"
    exit 1
fi

target_dir="$1"
shift

# Initialize variables
extensions=()
output_dir="out"
output_filename="context.txt" # Default filename
tree_file="tree.tmp"
context_body="body.tmp"

# Loop through remaining arguments
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
        -o|--output)
            if [[ -n "$2" && "$2" != -* ]]; then
                output_filename="$2"
                shift 2
            else
                echo "Error: -o requires a filename"
                exit 1
            fi
            ;;
        *)
            shift
            ;;
    esac
done

# Define final path after potential flag override
context_file="${output_dir}/${output_filename}"

# If no extensions were provided, default to txt
if [ ${#extensions[@]} -eq 0 ]; then
    extensions+=("-e" "txt")
fi

mkdir -p ${output_dir}
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
    ./aggregate -d "${target_dir}" "${extensions[@]}" -o "${context_body}"
else
    echo "Error: ./aggregate tool not found."
    rm -f "${tree_file}"
    exit 1
fi

# 3. Combine into final file
cat "${tree_file}" "${context_body}" > "${context_file}"

# 4. Cleanup
rm "${tree_file}" "${context_body}"

echo "Finished building ${context_file}"
