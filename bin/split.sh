#!/bin/bash

# Check if enough arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <file_to_split> <number_of_files>"
    exit 1
fi

# Get the arguments
file_to_split="$1"
number_of_files="$2"

# Extract filename (without extension)
filename=$(basename -- "${file_to_split}")
output_prefix="${filename%.*}"

# Calculate how many lines each split file should have
total_lines=$(wc -l < "${file_to_split}")
lines_per_file=$(echo "$total_lines / $number_of_files" | bc)

# Use split command to split the file, and specify the suffix of output files as .txt
split -l $lines_per_file -d --additional-suffix=.txt "${file_to_split}" "${output_prefix}"

echo "Split complete!"
