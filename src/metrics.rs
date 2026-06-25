// ระบบนี้คือ High-Speed Blockchain Banking System สำหรับ NDID (National Digital ID)
// ระบบพัฒนาขึ้นเพื่อรองรับการทำธุรกรรมธนาคารข้ามประเทศอย่างรวดเร็ว
// ภาษา: Rust, รันไทม์: Tokio async, โปรโตคอล: QUIC + TCP/TLS 1.3 Auto-Fallback
// ชั้นบริการ API: GraphQL (async-graphql) over Axum
// บล็อกเชน: Substrate (Private Permissioned Ledger)
// คริปโต: ED25519 (signing), AES-GCM (encryption), SHA-256 (hashing)

// โมดูลรวบรวมเมตริกสำหรับ Prometheus (KYC requests, P2P messages, blockchain retries)
use prometheus::{
    Encoder, IntCounterVec, Registry, TextEncoder, register_int_counter_vec_with_registry,
};
use std::sync::OnceLock;

// รีจิสทรีส่วนกลางสำหรับเมตริก Prometheus
pub fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(Registry::new)
}

// เมตริกนับจำนวนคำขอ KYC แยกตามธนาคารและสถานะ
pub fn kyc_requests() -> &'static IntCounterVec {
    static KYC_REQUESTS: OnceLock<IntCounterVec> = OnceLock::new();
    KYC_REQUESTS.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_kyc_requests_total",
            "Total number of KYC requests submitted",
            &["bank_code", "status"],
            registry()
        )
        .unwrap()
    })
}

// เมตริกนับจำนวนข้อความ P2P แยกตามทิศทาง ธนาคาร และสถานะ
pub fn p2p_messages() -> &'static IntCounterVec {
    static P2P_MESSAGES: OnceLock<IntCounterVec> = OnceLock::new();
    P2P_MESSAGES.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_p2p_messages_total",
            "Total number of P2P messages",
            &["direction", "bank_code", "status"],
            registry()
        )
        .unwrap()
    })
}

// เมตริกนับจำนวนการส่งธุรกรรมบล็อกเชนซ้ำ
pub fn blockchain_retries() -> &'static IntCounterVec {
    static BLOCKCHAIN_RETRIES: OnceLock<IntCounterVec> = OnceLock::new();
    BLOCKCHAIN_RETRIES.get_or_init(|| {
        register_int_counter_vec_with_registry!(
            "ndid_blockchain_retries_total",
            "Total number of Substrate blockchain retries",
            &["status"],
            registry()
        )
        .unwrap()
    })
}

// รวบรวมเมตริกทั้งหมดในรูปแบบข้อความ Prometheus (text/plain)
pub fn gather_metrics() -> Result<String, String> {
    // Access metrics to ensure they are registered
    let _ = kyc_requests();
    let _ = p2p_messages();
    let _ = blockchain_retries();

    let encoder = TextEncoder::new();
    let metric_families = registry().gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|e| format!("failed to encode metrics: {e}"))?;

    String::from_utf8(buffer).map_err(|e| format!("failed to convert metrics to UTF-8: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registration_and_gathering() {
        kyc_requests().with_label_values(&["SCB", "success"]).inc();
        p2p_messages()
            .with_label_values(&["out", "BBL", "sent"])
            .inc();
        blockchain_retries().with_label_values(&["failed"]).inc();

        let output = gather_metrics().expect("Failed to gather metrics");

        assert!(output.contains("ndid_kyc_requests_total"));
        assert!(output.contains("bank_code=\"SCB\""));
        assert!(output.contains("status=\"success\""));

        assert!(output.contains("ndid_p2p_messages_total"));
        assert!(output.contains("direction=\"out\""));

        assert!(output.contains("ndid_blockchain_retries_total"));
    }
}
