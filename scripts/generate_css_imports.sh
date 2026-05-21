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

# Copy third-party vendor libraries from node_modules to public/
VENDOR_DIR="public/vendor"
rm -rf "$VENDOR_DIR"

# Leaflet
mkdir -p "$VENDOR_DIR/leaflet"
cp thirdparty/node_modules/leaflet/dist/leaflet.js "$VENDOR_DIR/leaflet/"
cp thirdparty/node_modules/leaflet/dist/leaflet.css "$VENDOR_DIR/leaflet/"
cp -r thirdparty/node_modules/leaflet/dist/images "$VENDOR_DIR/leaflet/images"

# Mermaid (minified bundle only)
mkdir -p "$VENDOR_DIR/mermaid"
cp thirdparty/node_modules/mermaid/dist/mermaid.min.js "$VENDOR_DIR/mermaid/"

# MathJax (entry point + output directory for font loading)
mkdir -p "$VENDOR_DIR/mathjax"
cp thirdparty/node_modules/mathjax/tex-mml-chtml.js "$VENDOR_DIR/mathjax/"
cp -r thirdparty/node_modules/mathjax/output "$VENDOR_DIR/mathjax/"

# Prism (syntax highlighting)
mkdir -p "$VENDOR_DIR/prism/components"
cp thirdparty/node_modules/prismjs/prism.js "$VENDOR_DIR/prism/"
cp thirdparty/node_modules/prismjs/components/prism-rust.min.js "$VENDOR_DIR/prism/components/"
cp thirdparty/node_modules/prismjs/themes/prism-tomorrow.min.css "$VENDOR_DIR/prism/"

echo "Copied vendor libraries to $VENDOR_DIR."
