#!/usr/bin/env python3

import os

# Thai comment patterns
THAI_COMMENT_PATTERNS = [
    "// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback",
    "// ชั้นบริการ API: GraphQL (async-graphql) over Axum",
    "// บล็อกเชน: Substrate (Private Permissioned Ledger)",
    "// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)",
]

def cleanup_file(file_path):
    print(f"Cleaning up Thai comments in {file_path}")
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Find the first occurrence of Thai comments
    thai_comment_start = -1
    for i, line in enumerate(lines):
        if line.strip() == THAI_COMMENT_PATTERNS[0]:
            thai_comment_start = i
            break
    
    # If we found Thai comments, remove duplicates
    if thai_comment_start >= 0:
        # Find the end of Thai comments (next non-comment line after the pattern)
        end_index = thai_comment_start + 1
        while end_index < len(lines) and (lines[end_index].startswith('//') or lines[end_index].strip() == ''):
            end_index += 1
        
        # Remove duplicate Thai comments
        new_lines = []
        i = 0
        while i < len(lines):
            if i == thai_comment_start:
                # Add the first occurrence
                new_lines.append(lines[i])
                i += 1
                # Skip subsequent duplicate Thai comments
                while i < len(lines) and lines[i].strip() == '':
                    new_lines.append(lines[i])
                    i += 1
                for pattern in THAI_COMMENT_PATTERNS[1:]:
                    while i < len(lines) and lines[i].strip() == pattern:
                        new_lines.append(lines[i])
                        i += 1
            else:
                new_lines.append(lines[i])
                i += 1
        
        # Write the cleaned content back to the file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)

def main():
    # Clean up all Rust source files
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith('.rs') and 'target' not in root and '.git' not in root:
                file_path = os.path.join(root, file)
                cleanup_file(file_path)

if __name__ == '__main__':
    main()
