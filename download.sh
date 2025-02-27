#!/bin/bash

file_name="knowledge_archive.tar.bz2"
output_file="knowledge_archive.tar.zst"
download_url="https://figshare.com/ndownloader/files/42860026"
expected_blake3="ab37f79c537df1c22479cf5a7a72f76bb005cabae8f09cb23e5301227a42929e"
unzipped_blake3="7467a1979b944b168635b47854913b19fa637c7f9ecf849d5052dbc83c759d5f"

dependencies=("aria2c" "b3sum" "bzcat" "zstd")

for dependency in "${dependencies[@]}"; do
  if ! command -v "$dependency" &> /dev/null; then
    echo "$dependency is not installed."
    exit 1
  fi
done

if [ ! -f "$file_name" ]; then
  echo "Downloading $file_name..."
  aria2c -c "$download_url"
  if [ $? -ne 0 ]; then
    echo "Error downloading $file_name"
    exit 1
  fi
  echo "Downloaded $file_name successfully."
else
  echo "$file_name already exists."
fi

if b3sum -c <<< "$expected_blake3  $file_name"; then
  echo "File integrity verified."
else
  echo "File verification failed."
  exit 1
fi

# Check if the zstd file exists, recompress if it doesn't
if [ ! -f "$output_file" ]; then
    echo "$output_file not found. Recompressing from $file_name..."
    bzcat "$file_name" | zstd -19 -T0 -o "$output_file"
    if [ $? -ne 0 ]; then
        echo "Error recompressing $file_name to $output_file"
        exit 1
    fi
    echo "Conversion completed: $output_file (Zstandard -19)"
else
    echo "$output_file already exists."
fi

# Verify the unzipped content
echo "Verifying unzipped content..."
actual_blake3=$(zstdcat "$output_file" | b3sum -)

if [ "$actual_blake3" == "$unzipped_blake3  -" ]; then
    echo "Unzipped file integrity verified."
else
    echo "Unzipped file verification failed."
    echo "Expected: $unzipped_blake3"
    echo "Actual: $actual_blake3"
    exit 1
fi
