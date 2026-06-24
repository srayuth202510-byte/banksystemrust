#!/usr/bin/env python3

import os

# All 6 Thai comment lines that should appear once at the top
THAI_COMMENTS = [
    "// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)",
    "// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว",
    "// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback",
    "// ชั้นบริการ API: GraphQL (async-graphql) over Axum",
    "// บล็อกเชน: Substrate (Private Permissioned Ledger)",
    "// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)",
]

# Keywords to identify Thai comment lines (partial matches)
THAI_KEYWORDS = ["ภาษา", "ชั้นบริการ", "บล็อกเชน", "คริปโต", "ระบบนี้คือ", "ระบบพัฒนาขึ้น"]

def cleanup_file(file_path):
    print(f"Cleaning up Thai comments in {file_path}")
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Remove all Thai comment lines from the file
    new_lines = []
    for line in lines:
        stripped = line.strip()
        # Check if this line is a Thai comment
        is_thai_comment = stripped.startswith("//") and any(keyword in stripped for keyword in THAI_KEYWORDS)
        if not is_thai_comment:
            new_lines.append(line)
    
    # Remove leading blank lines
    while new_lines and new_lines[0].strip() == "":
        new_lines.pop(0)
    
    # Now add the full Thai comment block at the beginning
    final_lines = []
    for comment in THAI_COMMENTS:
        final_lines.append(comment + "\n")
    final_lines.append("\n")
    final_lines.extend(new_lines)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(final_lines)

def main():
    # Clean up all Rust source files
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith('.rs') and 'target' not in root and '.git' not in root:
                file_path = os.path.join(root, file)
                cleanup_file(file_path)

if __name__ == '__main__':
    main()
