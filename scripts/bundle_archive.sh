#!/bin/bash

# Configuration
ARCHIVE_DIR="./build"
# Get the current directory name
FOLDER_NAME=$(basename "$(pwd)")
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
ARCHIVE_NAME="${FOLDER_NAME}_${TIMESTAMP}.tar.gz"
EXCLUDES=(
    "target"
    "dist"
    ".git"
    ".trunk"
    "build_error.log"
    "build_output.log"
)

# Create build directory if it doesn't exist
mkdir -p "$ARCHIVE_DIR"

echo "📦 Archiving codebase to ${ARCHIVE_DIR}/${ARCHIVE_NAME}..."

# Build the exclude arguments for tar
EXCLUDE_ARGS=()
for item in "${EXCLUDES[@]}"; do
    EXCLUDE_ARGS+=("--exclude=$item")
done

# Create the compressed archive
# Use -C . to avoid including the full path in the archive
if tar -czf "${ARCHIVE_DIR}/${ARCHIVE_NAME}" "${EXCLUDE_ARGS[@]}" .; then
    echo "✅ Successfully archived codebase!"
    echo "Location: ${ARCHIVE_DIR}/${ARCHIVE_NAME}"
    
    # Calculate size of the archive
    SIZE=$(du -h "${ARCHIVE_DIR}/${ARCHIVE_NAME}" | cut -f1)
    echo "Archive Size: $SIZE"
else
    echo "❌ Failed to create archive."
    exit 1
fi
