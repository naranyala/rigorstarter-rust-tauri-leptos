#!/bin/bash

# expose.sh - Identify the largest Rust files in Project

echo "--------------------------------------------------------------------------------"
echo "🔍 Top 15 Largest Rust Files in Project"
echo "--------------------------------------------------------------------------------"
echo -e "Lines\tFile Path"
echo -e "-----\t--------------------------------------------------------------------"

# Find all .rs files, count lines, sort them numerically descending, and take the top 15
find src -name "*.rs" -exec wc -l {} + | grep -v " total$" | sort -rn | head -n 15 | while read -r line; do
    # Extract line count and file path
    count=$(echo "$line" | awk '{print $1}')
    file=$(echo "$line" | awk '{$1=""; print $0}' | sed 's/^[ \t]*//')

    echo -e "$count\t$file"
done

echo "--------------------------------------------------------------------------------"
echo "💡 General Refactoring Tips:"
echo "1. Extract complex closures into named functions."
echo "2. Move domain-specific logic to src/core/logic.rs."
echo "3. Use separate files for large components."
echo "4. Consolidate redundant state management into dedicated services."
echo "--------------------------------------------------------------------------------"
