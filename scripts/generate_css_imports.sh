#!/bin/bash

# Configuration
CSS_DIR="styles"
TEMPLATE_FILE="index.html.default"
TARGET_FILE="index.html"
PLACEHOLDER="<!-- CSS_IMPORTS -->"

# Check if template file exists
if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Error: Template $TEMPLATE_FILE not found!"
    exit 1
fi

# Temporary file for generation
TMP_FILE="index.html.tmp"
> "$TMP_FILE"

# Process the template file
while IFS= read -r line || [ -n "$line" ]; do
    if [[ "$line" == *"$PLACEHOLDER"* ]]; then
        # 1. Always put base.css first for variable inheritance
        if [ -f "$CSS_DIR/base.css" ]; then
            echo "    <link data-trunk rel=\"css\" href=\"$CSS_DIR/base.css\" />" >> "$TMP_FILE"
        fi
        
        # 2. Put other CSS files alphabetically
        for file in $(ls $CSS_DIR/*.css 2>/dev/null | sort); do
            if [[ "$file" != *"/base.css" ]]; then
                echo "    <link data-trunk rel=\"css\" href=\"$file\" />" >> "$TMP_FILE"
            fi
        done
    else
        echo "$line" >> "$TMP_FILE"
    fi
done < "$TEMPLATE_FILE"

# Atomically move to target file
mv "$TMP_FILE" "$TARGET_FILE"

echo "Successfully generated $TARGET_FILE from $TEMPLATE_FILE."
