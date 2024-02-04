#!/bin/bash

# Check the number of parameters
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 directory"
    exit 1
fi

# Get the input and output directory path from command line argument
dir="$1"

# Find the image file in the directory (supports jpg, png, jpeg formats)
image_file=$(find "$dir" -type f \( -iname \*.jpg -o -iname \*.png -o -iname \*.jpeg \))

# Loop through all MP3 files in the directory
for audio_file in "$dir"/*.mp3
do
    # Get the base name of the audio file (without path and extension)
    base_name=$(basename "$audio_file" .mp3)

    # Specify the output file path
    output_file="$dir/$base_name.mp4"

    # Use ffmpeg to merge the image and audio into a video, and run it in the background
    ffmpeg -loop 1 -i "$image_file" -i "$audio_file" -c:v libx264 -preset superfast -crf 30 -tune stillimage -c:a copy -shortest "$output_file" &
done

# Wait for all background tasks to complete
wait
