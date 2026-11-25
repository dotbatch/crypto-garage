// The exported API, run natively. The ciphers' known-answer and bit-flip tests live in
// apps/aead; these check what this crate adds: algorithm names, per-playground keys, nonce
// length checks and the page's tamper and nonce-reuse paths.

use aead_wasm::{Playground, seal_reusing_nonce};

const ALGORITHMS: [&str; 2] = ["aes-256-gcm", "chacha20-poly1305"];
const TAG_LEN: usize = 16;

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

#[test]
fn round_trip_recovers_utf8_text() {
    for alg in ALGORITHMS {
        let playground = Playground::new(alg).unwrap();
        let sealed = playground.seal("header", "naïve café").unwrap();
        assert_eq!(sealed.nonce().len(), 12, "{alg}");
        assert_eq!(
            sealed.ciphertext().len(),
            "naïve café".len() + TAG_LEN,
            "{alg}"
        );
        let plain = playground
            .open(&sealed.nonce(), "header", &sealed.ciphertext())
            .unwrap();
        assert_eq!(plain, "naïve café".as_bytes(), "{alg}");
    }
}

#[test]
fn unknown_algorithm_is_refused() {
    assert!(Playground::new("aes-128-cbc").is_err());
    assert!(seal_reusing_nonce("aes-128-cbc", "a", "b").is_err());
}

// What the page's tamper button does: change one byte, in the ciphertext body or the tag.
#[test]
fn tampered_byte_is_rejected() {
    for alg in ALGORITHMS {
        let playground = Playground::new(alg).unwrap();
        let sealed = playground.seal("", "attack at dawn").unwrap();
        for index in [0, sealed.ciphertext().len() - 1] {
            let mut tampered = sealed.ciphertext();
            tampered[index] ^= 0x01;
            let result = playground.open(&sealed.nonce(), "", &tampered);
            assert!(
                result.unwrap_err().starts_with("rejected"),
                "{alg} byte {index}"
            );
        }
    }
}

#[test]
fn changed_aad_is_rejected() {
    for alg in ALGORITHMS {
        let playground = Playground::new(alg).unwrap();
        let sealed = playground.seal("to: alice", "hi").unwrap();
        let result = playground.open(&sealed.nonce(), "to: mallory", &sealed.ciphertext());
        assert!(result.is_err(), "{alg}");
    }
}

#[test]
fn each_playground_has_its_own_key() {
    for alg in ALGORITHMS {
        let first = Playground::new(alg).unwrap();
        let second = Playground::new(alg).unwrap();
        let sealed = first.seal("", "secret").unwrap();
        assert!(
            second
                .open(&sealed.nonce(), "", &sealed.ciphertext())
                .is_err(),
            "{alg}"
        );
    }
}

#[test]
fn nonce_of_wrong_length_is_refused() {
    let playground = Playground::new("aes-256-gcm").unwrap();
    let sealed = playground.seal("", "hi").unwrap();
    let short = &sealed.nonce()[..11];
    assert_eq!(
        playground.open(short, "", &sealed.ciphertext()),
        Err("the nonce must be 12 bytes".to_string())
    );
}

// The deliberately broken path: two messages under one key and nonce leak p1 ^ p2.
#[test]
fn reused_nonce_leaks_plaintext_xor() {
    // Lengths that cross a 16-byte block boundary, and differ, as typed messages will.
    let p1 = "Transfer $100 to account 1111, then confirm";
    let p2 = "Meet at the north gate at 6pm";
    for alg in ALGORITHMS {
        let reused = seal_reusing_nonce(alg, p1, p2).unwrap();
        assert_eq!(reused.c1().len(), p1.len(), "{alg}: tag must be dropped");
        assert_ne!(reused.c1(), p1.as_bytes(), "{alg}");
        assert_eq!(
            xor(&reused.c1(), &reused.c2()),
            xor(p1.as_bytes(), p2.as_bytes()),
            "{alg}"
        );
    }
}
