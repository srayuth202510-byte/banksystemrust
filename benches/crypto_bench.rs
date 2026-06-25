// การทดสอบสมรรถนะ (Benchmarks) สำหรับฟังก์ชันการเข้ารหัส: ED25519, AES-256-GCM, SHA-256
use banksystemrust::crypto::{KeyPair, decrypt, encrypt, hash_hex, sign, verify};
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::RngCore;

// ทดสอบสมรรถนะการลงนามและตรวจสอบลายเซ็น ED25519
fn bench_ed25519(c: &mut Criterion) {
    let mut group = c.benchmark_group("ED25519");
    let keypair = KeyPair::generate().unwrap();
    let payload = b"KYC:BBL:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    group.bench_function("sign", |b| {
        b.iter(|| sign(black_box(payload), black_box(&keypair)).unwrap())
    });

    let signed = sign(payload, &keypair).unwrap();
    group.bench_function("verify", |b| b.iter(|| verify(black_box(&signed)).unwrap()));
    group.finish();
}

// ทดสอบสมรรถนะการเข้ารหัสและถอดรหัส AES-256-GCM สำหรับ payload 1KB
fn bench_aes256gcm(c: &mut Criterion) {
    let mut group = c.benchmark_group("AES-256-GCM");
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let payload = vec![0u8; 1024]; // 1KB payload

    group.bench_function("encrypt_1kb", |b| {
        b.iter(|| encrypt(black_box(&payload), black_box(&key)).unwrap())
    });

    let encrypted = encrypt(&payload, &key).unwrap();
    group.bench_function("decrypt_1kb", |b| {
        b.iter(|| decrypt(black_box(&encrypted), black_box(&key)).unwrap())
    });
    group.finish();
}

// ทดสอบสมรรถนะการแฮช SHA-256 สำหรับข้อมูล KYC
fn bench_sha256(c: &mut Criterion) {
    let mut group = c.benchmark_group("SHA-256");
    let data = b"{\"national_id\":\"1234567890123\",\"full_name\":\"John Doe\",\"date_of_birth\":\"1990-01-01\",\"bank_code\":\"BBL\",\"timestamp\":1718000000}";

    group.bench_function("hash_kyc_data", |b| b.iter(|| hash_hex(black_box(data))));
    group.finish();
}

criterion_group!(benches, bench_ed25519, bench_aes256gcm, bench_sha256);
criterion_main!(benches);
