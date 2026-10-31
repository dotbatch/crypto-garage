# Roadmap

Candidate projects, grouped by theme. Order within a group is a rough suggested build order. Standards versions are as of October 2026; recheck before starting a project.

## 1. Proven foundations

| Project | What it shows | Standards |
|---|---|---|
| AEAD playground | AES-256-GCM vs ChaCha20-Poly1305; live demo of what nonce reuse breaks | SP 800-38D, RFC 8439 |
| Key exchange + KDF | X25519 handshake feeding HKDF into session keys | RFC 7748, RFC 5869 |
| Signatures | Ed25519 and ECDSA P-256 sign/verify, deterministic nonces, malleability | FIPS 186-5, RFC 8032 |
| Password storage | Argon2id parameter tuning vs bcrypt/PBKDF2, with timing charts | RFC 9106, SP 800-63B |
| Mini secure channel | A Noise-protocol (`Noise_XX`) chat between two processes | Noise spec rev 34 |

## 2. Post-quantum (the "latest")

| Project | What it shows | Standards |
|---|---|---|
| ML-KEM explorer | Key encapsulation, key/ciphertext sizes, speed vs X25519 | FIPS 203 |
| ML-DSA and SLH-DSA signer | Lattice vs hash-based signatures, size and speed trade-offs | FIPS 204, FIPS 205 |
| Hybrid TLS demo | `X25519MLKEM768` key exchange in a real TLS 1.3 handshake | IETF hybrid KEM drafts, OpenSSL 3.5+ |
| Crypto-agility harness | Swap algorithms by config; inventory what a codebase uses (a CBOM) | NIST IR 8547 (PQC transition) |
| Watch list | FN-DSA (Falcon, draft FIPS 206) and HQC (selected 2025) once final | — |

## 3. Embedded and IoT

| Project | What it shows | Standards |
|---|---|---|
| Lightweight crypto on a microcontroller | Ascon-AEAD128 vs AES-GCM on a microcontroller-class target (RP2040 / STM32) | NIST SP 800-232 |
| Secure boot and firmware signing | Signed image chain, rollback protection, PQC-signed firmware with ML-DSA / LMS | SP 800-193, SP 800-208 |
| Signed OTA updates with TUF | Role separation, threshold signatures, key rotation and compromise recovery for device updates | TUF specification 1.0 |

## 4. Other ideas

- Zero-knowledge proof demo (Schnorr identification, then a small zk-SNARK)
- Shamir secret sharing for key backup
- Merkle trees and transparency logs (the Sigstore / Certificate Transparency idea)
- Side-channel lab: timing attack on a naive comparison, then the constant-time fix

## Cross-cutting work

- [x] Pick languages (Rust for core, embedded and Wasm demos; Python for teaching notebooks; see `CLAUDE.md`)
- [x] Choose a license (Apache-2.0)
- [ ] CI: build, unit tests, known-answer tests, Wycheproof, `cargo audit` / `pip-audit`
- [ ] Shared test-vector fetcher under `vectors/`
- [ ] Doc template per app: goal, threat model, standard cited, how to run, limitations
- [ ] Showcase site and PQ TLS endpoint: see [docs/hosting-plan.md](docs/hosting-plan.md) and [docs/deployment-plan.md](docs/deployment-plan.md)
