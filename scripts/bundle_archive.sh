#!/bin/bash

# Configuration
ARCHIVE_DIR="./build"
FOLDER_NAME=$(basename "$(pwd)")
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Compressor: gzip, pigz (parallel), or zstd
COMPRESSOR="gzip"

# Items to exclude from the archive
EXCLUDES=(
    "target"
    "dist"
    "build"
    ".trunk"
    "node_modules"
    "*.log"
    "src-tauri/gen/schemas"
)

# Shrink git pack before archiving
git gc --aggressive 2>/dev/null

# Prune old archives (keep last 5)
ls -t "$ARCHIVE_DIR"/*.tar.* 2>/dev/null | tail -n +6 | xargs rm -f 2>/dev/null

mkdir -p "$ARCHIVE_DIR"

# Map compressor to command + extension
case "$COMPRESSOR" in
    pigz)  COMPRESS_CMD="pigz --best"   SUFFIX=".tar.gz"  ;;
    zstd)  COMPRESS_CMD="zstd -19"       SUFFIX=".tar.zst" ;;
    *)     COMPRESS_CMD="gzip --best"    SUFFIX=".tar.gz"  ;;
esac

ARCHIVE_NAME="${FOLDER_NAME}_${TIMESTAMP}${SUFFIX}"
EXCLUDE_ARGS=()
for item in "${EXCLUDES[@]}"; do
    EXCLUDE_ARGS+=("--exclude=$item")
done

echo "📦 Archiving codebase → ${ARCHIVE_DIR}/${ARCHIVE_NAME}"

if tar -cf - "${EXCLUDE_ARGS[@]}" . | $COMPRESS_CMD > "${ARCHIVE_DIR}/${ARCHIVE_NAME}"; then
    SIZE=$(du -h "${ARCHIVE_DIR}/${ARCHIVE_NAME}" | cut -f1)
    echo "✅ Done — $SIZE"

    # Quick breakdown of excluded items that exist
    for d in "${EXCLUDES[@]}"; do
        [[ "$d" == *.log ]] && continue
        if [ -d "$d" ] 2>/dev/null; then
            echo "  (excluded $d — $(du -sh "$d" 2>/dev/null | cut -f1))"
        elif [ -f "$d" ] 2>/dev/null; then
            echo "  (excluded $d — $(du -h "$d" 2>/dev/null | cut -f1))"
        fi
    done
else
    echo "❌ Failed to create archive."
    exit 1
fi
