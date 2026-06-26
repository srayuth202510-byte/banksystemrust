# Project Code Graph Index

Welcome to the Obsidian Code Graph for **banksystemrust**.

## Modules
- [[blockchain|blockchain.rs]]
- [[config|config.rs]]
- [[crypto|crypto.rs]]
- [[identity|identity.rs]]
- [[main|main.rs]]
- [[metrics|metrics.rs]]
- [[network_mod|network/mod.rs]]
- [[network_quic_channel|network/quic_channel.rs]]
- [[network_tcp_channel|network/tcp_channel.rs]]
- [[network_tls|network/tls.rs]]
- [[p2p_quic|p2p_quic.rs]]
- [[redis_cache|redis_cache.rs]]
- [[schema|schema.rs]]


## System Error Dictionary
### [[Types/P2pError|P2pError]]
- [[P2pError_Network|Network]] : `"network error: {0}"`
- [[P2pError_Crypto|Crypto]] : `"crypto error: {0}"`
- [[P2pError_PeerNotFound|PeerNotFound]] : `"peer not found: {0}"`
- [[P2pError_HandshakeFailed|HandshakeFailed]] : `"handshake failed: {0}"`
- [[P2pError_TlsError|TlsError]] : `"tls error: {0}"`
### [[Types/IdentityError|IdentityError]]
- [[IdentityError_ValidationFailed|ValidationFailed]] : `"validation failed: {0}"`
- [[IdentityError_HashMismatch|HashMismatch]] : `"hash mismatch: {0}"`
- [[IdentityError_NotFound|NotFound]] : `"record not found: {0}"`
- [[IdentityError_Unauthorized|Unauthorized]] : `"unauthorized: {0}"`
- [[IdentityError_Crypto|Crypto]] : `"crypto error: {0}"`
### [[Types/AppConfigError|AppConfigError]]
- [[AppConfigError_Message|Message]] : `"{0}"`
- [[AppConfigError_Config|Config]] : `"config error: {0}"`
### [[Types/BlockchainError|BlockchainError]]
- [[BlockchainError_NodeUnreachable|NodeUnreachable]] : `"node unreachable: {0}"`
- [[BlockchainError_TransactionFailed|TransactionFailed]] : `"transaction failed: {0}"`
- [[BlockchainError_Timeout|Timeout]] : `"timeout after {0}s"`
- [[BlockchainError_ConsensusFailed|ConsensusFailed]] : `"consensus not reached"`
- [[BlockchainError_InvalidTransaction|InvalidTransaction]] : `"invalid transaction: {0}"`
- [[BlockchainError_Http|Http]] : `"http error: {0}"`
- [[BlockchainError_Crypto|Crypto]] : `"crypto error: {0}"`
- [[BlockchainError_DatabaseError|DatabaseError]] : `"database error: {0}"`
### [[Types/RedisCacheError|RedisCacheError]]
- [[RedisCacheError_Client|Client]] : `"redis client error: {0}"`
- [[RedisCacheError_Timeout|Timeout]] : `"redis operation timed out after {0}ms"`
- [[RedisCacheError_Serialization|Serialization]] : `"serialization error: {0}"`
- [[RedisCacheError_SecretLoad|SecretLoad]] : `"redis secret load failed: {0}"`
### [[Types/CryptoError|CryptoError]]
- [[CryptoError_SigningFailed|SigningFailed]] : `"signing failed: {0}"`
- [[CryptoError_VerificationFailed|VerificationFailed]] : `"verification failed: {0}"`
- [[CryptoError_EncryptionFailed|EncryptionFailed]] : `"encryption failed: {0}"`
- [[CryptoError_DecryptionFailed|DecryptionFailed]] : `"decryption failed: {0}"`
- [[CryptoError_KeyGenerationFailed|KeyGenerationFailed]] : `"key generation failed: {0}"`
- [[CryptoError_InvalidKey|InvalidKey]] : `"invalid key: {0}"`
- [[CryptoError_HsmError|HsmError]] : `"hsm error: {0}"`
### [[Types/TlsError|TlsError]]
- [[TlsError_CertGeneration|CertGeneration]] : `"certificate generation failed: {0}"`
- [[TlsError_CertLoading|CertLoading]] : `"certificate loading failed: {0}"`
- [[TlsError_InvalidKey|InvalidKey]] : `"invalid key: {0}"`
### [[Types/NetworkError|NetworkError]]
- [[NetworkError_QuicFailed|QuicFailed]] : `"quic connection failed: {0}"`
- [[NetworkError_TcpFailed|TcpFailed]] : `"tcp connection failed: {0}"`
- [[NetworkError_BothFailed|BothFailed]] : `"both protocols failed"`
- [[NetworkError_Timeout|Timeout]] : `"timeout"`
- [[NetworkError_TlsError|TlsError]] : `"tls error: {0}"`
- [[NetworkError_ConnectionLost|ConnectionLost]] : `"connection lost: {0}"`

## Project Statistics
- **Total Files Scanned:** 13
- **Total Types:** 47
- **Total Functions:** 137
- **Total Error Variants:** 40
