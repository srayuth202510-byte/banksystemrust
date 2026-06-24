#!/usr/bin/env python3

import os
import re

# Thai comment patterns that should appear once at the top
THAI_COMMENTS = [
    "// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)",
    "// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว",
    "// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback",
    "// ชั้นบริการ API: GraphQL (async-graphql) over Axum",
    "// บล็อกเชน: Substrate (Private Permissioned Ledger)",
    "// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)",
]

def cleanup_file(file_path):
    print(f"Cleaning up Thai comments in {file_path}")
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find all occurrences of the Thai comment blocks
    # The pattern matches a block of Thai comments followed by blank lines
    thai_block_pattern = re.compile(
        r'(// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID \(National Digital ID\)\n'
        r'// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว\n'
        r'// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC \+ TCP/TLS 1\.3 Auto-Fallback\n'
        r'// ชั้นบริการ API: GraphQL \(async-graphql\) over Axum\n'
        r'// บล็อกเชน: Substrate \(Private Permissioned Ledger\)\n'
        r'// คริปโต: ED25519 \(signing\), AES-GCM \(encryption\), SHA-256 \(hashing\)\n)'
    )
    
    # Find all matches
    matches = list(thai_block_pattern.finditer(content))
    
    if len(matches) <= 1:
        return
    
    # Keep only the first occurrence, remove all others
    new_content = content[:matches[0].start()]
    
    # Add the first occurrence
    new_content += matches[0].group(0)
    new_content += "\n"
    
    # Add the rest of the content after the last match
    last_match_end = matches[-1].end()
    new_content += content[last_match_end:]
    
    # Write the cleaned content back to the file
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(new_content)

def main():
    # Clean up all Rust source files
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith('.rs') and 'target' not in root and '.git' not in root:
                file_path = os.path.join(root, file)
                cleanup_file(file_path)

if __name__ == '__main__':
    main()
