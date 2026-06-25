// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// โมดูลเชื่อมต่อบล็อกเชน Substrate สำหรับบันทึกธุรกรรม
pub mod blockchain;
// โมดูลจัดการการตั้งค่าระบบ
pub mod config;
// โมดูลเข้ารหัส ED25519, AES-GCM, SHA-256
pub mod crypto;
// โมดูลข้อมูลประจำตัว NDID และ KYC
pub mod identity;
// โมดูลเก็บสถิติและเมตริกของระบบ
pub mod metrics;
// โมดูลเครือข่าย P2P (QUIC + TCP/TLS 1.3)
pub mod network;
// โมดูลจัดการโหนด P2P สำหรับส่งข้อมูล KYC ระหว่างธนาคาร
pub mod p2p_quic;
// โมดูลเชื่อมต่อ Redis Cache
pub mod redis_cache;
// โมดูลนิยาม GraphQL Schema (Query, Mutation, Subscription)
pub mod schema;
