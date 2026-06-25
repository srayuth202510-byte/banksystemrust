// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// นำเข้าไลบรารีสำหรับ Serialize/Deserialize และการจัดการข้อผิดพลาด
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::crypto;

// ข้อผิดพลาดที่เกี่ยวกับข้อมูลประจำตัว NDID
#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("validation failed: {0}")]
    ValidationFailed(String),
    #[error("hash mismatch: {0}")]
    HashMismatch(String),
    #[error("record not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("crypto error: {0}")]
    Crypto(#[from] crypto::CryptoError),
}

// สถานะของข้อมูลประจำตัว (Pending/Approved/Rejected/Revoked)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityStatus {
    Pending,
    Approved,
    Rejected,
    Revoked,
}

// บันทึกข้อมูลประจำตัวบนบล็อกเชน
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRecord {
    pub request_id: String,        // รหัสอ้างอิงคำขอ
    pub status: IdentityStatus,    // สถานะปัจจุบัน
    pub identity_hash: String,     // ค่าแฮช SHA-256 ของข้อมูล
    pub timestamp: i64,            // เวลาที่สร้าง (Unix timestamp)
    pub bank_code: String,         // รหัสธนาคารเจ้าของข้อมูล
    pub active_protocol: String,   // โปรโตคอลที่ใช้ส่งข้อมูล (QUIC/TCP)
}

// ข้อมูล KYC (Know Your Customer) จากลูกค้าธนาคาร
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycData {
    pub national_id: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub bank_code: String,
    pub timestamp: i64,
}

impl KycData {
    // คำนวณค่าแฮช SHA-256 ของข้อมูล KYC เพื่อใช้เป็นลายเซ็นประจำตัว
    pub fn compute_hash(&self) -> Result<String, IdentityError> {
        let serialized = serde_json::to_vec(self)
            .map_err(|e| IdentityError::ValidationFailed(format!("serialization failed: {e}")))?;
        let hash = crypto::hash(&serialized);
        Ok(hex::encode(hash))
    }

    // ปิดบังข้อมูลส่วนบุคคล (PII) โดยคงเฉพาะแฮชและข้อมูลที่ไม่ระบุตัวตน
    pub fn anonymize(&self) -> Result<AnonymizedKyc, IdentityError> {
        Ok(AnonymizedKyc {
            identity_hash: self.compute_hash()?,
            bank_code: self.bank_code.clone(),
            timestamp: self.timestamp,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedKyc {
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
}

// ตรวจสอบความถูกต้องของค่าแฮชโดยใช้ Constant-Time Comparison (ป้องกัน timing attack)
pub fn validate_identity_hash(kyc: &KycData, expected_hash: &str) -> Result<bool, IdentityError> {
    let actual_hash = kyc.compute_hash()?;
    use subtle::ConstantTimeEq;
    Ok(actual_hash
        .as_bytes()
        .ct_eq(expected_hash.as_bytes())
        .into())
}

// สร้างบันทึกข้อมูลประจำตัวในระบบ NDID
pub fn create_identity_record(
    request_id: String,
    kyc: &KycData,
    bank_code: String,
    protocol: String,
) -> Result<IdentityRecord, IdentityError> {
    let identity_hash = kyc.compute_hash()?;
    Ok(IdentityRecord {
        request_id,
        status: IdentityStatus::Pending,
        identity_hash,
        timestamp: chrono::Utc::now().timestamp(),
        bank_code,
        active_protocol: protocol,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kyc() -> KycData {
        KycData {
            national_id: "1234567890123".into(),
            full_name: "สมชาย ใจดี".into(),
            date_of_birth: "1990-01-01".into(),
            bank_code: "BBL".into(),
            timestamp: 1700000000,
        }
    }

    #[test]
    fn test_kyc_hash_consistency() {
        let kyc = sample_kyc();
        let hash1 = kyc.compute_hash().unwrap();
        let hash2 = kyc.compute_hash().unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_anonymize_removes_pii() {
        let kyc = sample_kyc();
        let anon = kyc.anonymize().unwrap();
        assert!(!anon.identity_hash.contains("1234567890123"));
        assert!(!anon.identity_hash.contains("สมชาย"));
    }

    #[test]
    fn test_create_identity_record() {
        let kyc = sample_kyc();
        let record =
            create_identity_record("req-001".into(), &kyc, "BBL".into(), "QUIC".into()).unwrap();
        assert!(matches!(record.status, IdentityStatus::Pending));
        assert_eq!(record.identity_hash.len(), 64);
    }
}
