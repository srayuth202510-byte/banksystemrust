use serde::{Deserialize, Serialize};
use thiserror::Error;


use crate::crypto;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityStatus {
    Pending,
    Approved,
    Rejected,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRecord {
    pub request_id: String,
    pub status: IdentityStatus,
    pub identity_hash: String,
    pub timestamp: i64,
    pub bank_code: String,
    pub active_protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KycData {
    pub national_id: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub bank_code: String,
    pub timestamp: i64,
}

impl KycData {
    pub fn compute_hash(&self) -> String {
        let serialized = serde_json::to_vec(self).unwrap_or_default();
        let hash = crypto::hash(&serialized);
        hex::encode(hash)
    }

    pub fn anonymize(&self) -> AnonymizedKyc {
        AnonymizedKyc {
            identity_hash: self.compute_hash(),
            bank_code: self.bank_code.clone(),
            timestamp: self.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedKyc {
    pub identity_hash: String,
    pub bank_code: String,
    pub timestamp: i64,
}

pub fn validate_identity_hash(kyc: &KycData, expected_hash: &str) -> Result<bool, IdentityError> {
    let actual_hash = kyc.compute_hash();
    use subtle::ConstantTimeEq;
    Ok(actual_hash.as_bytes().ct_eq(expected_hash.as_bytes()).into())
}

pub fn create_identity_record(
    request_id: String,
    kyc: &KycData,
    bank_code: String,
    protocol: String,
) -> IdentityRecord {
    let identity_hash = kyc.compute_hash();
    IdentityRecord {
        request_id,
        status: IdentityStatus::Pending,
        identity_hash,
        timestamp: chrono::Utc::now().timestamp(),
        bank_code,
        active_protocol: protocol,
    }
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
        let hash1 = kyc.compute_hash();
        let hash2 = kyc.compute_hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_anonymize_removes_pii() {
        let kyc = sample_kyc();
        let anon = kyc.anonymize();
        assert!(!anon.identity_hash.contains("1234567890123"));
        assert!(!anon.identity_hash.contains("สมชาย"));
    }

    #[test]
    fn test_create_identity_record() {
        let kyc = sample_kyc();
        let record = create_identity_record(
            "req-001".into(),
            &kyc,
            "BBL".into(),
            "QUIC".into(),
        );
        assert!(matches!(record.status, IdentityStatus::Pending));
        assert_eq!(record.identity_hash.len(), 64);
    }
}
