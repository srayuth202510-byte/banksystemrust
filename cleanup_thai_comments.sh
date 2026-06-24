#!/bin/bash

# Clean up duplicated Thai comments in Rust source files

# Function to clean up duplicated Thai comments in a file
cleanup_thai_comments() {
    local file=$1
    echo "Cleaning up Thai comments in $file"
    
    # Use sed to remove duplicate lines that start with the same Thai comment pattern
    sed -i '0,/\/\/ ภาษา:/!b; :a; n; /^\/\/ ภาษา:/!ba; d' "$file"
}

# Clean up duplicated Thai comments in all Rust source files
for file in src/*.rs src/network/*.rs tests/integration_test.rs; do
    if [ -f "$file" ]; then
        cleanup_thai_comments "$file"
    fi
done

