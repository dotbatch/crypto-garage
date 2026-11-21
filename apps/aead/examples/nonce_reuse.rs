//! DELIBERATELY BROKEN: FAILURE-MODE DEMO. DO NOT COPY.
//!
//! This file encrypts two messages under the same key AND the same nonce, which both AES-GCM
//! and ChaCha20-Poly1305 forbid. It exists to show what an eavesdropper gets when that happens.
//! Nothing outside this file imports it, and the crate's own `seal` never lets a caller pick
//! the nonce. See "How it breaks" in the app README for the explanation.
//!
//! Run it with `cargo run -p aead-demo --example nonce_reuse`.

use aead_demo::{Algorithm, KEY_LEN, NONCE_LEN, generate_key};
use aes_gcm::Aes256Gcm;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::{Aead, KeyInit};

/// The bug being demonstrated: one fixed nonce for every message.
const REUSED_NONCE: [u8; NONCE_LEN] = [0; NONCE_LEN];

/// Encrypts with the reused nonce and returns the ciphertext without its tag.
fn broken_encrypt(alg: Algorithm, key: &[u8; KEY_LEN], plaintext: &[u8]) -> Vec<u8> {
    let nonce = REUSED_NONCE.into();
    let mut sealed = match alg {
        Algorithm::Aes256Gcm => Aes256Gcm::new(key.into()).encrypt(&nonce, plaintext),
        Algorithm::ChaCha20Poly1305 => ChaCha20Poly1305::new(key.into()).encrypt(&nonce, plaintext),
    }
    .unwrap();
    sealed.truncate(plaintext.len());
    sealed
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

/// What the attacker learns from two ciphertexts that shared a key and nonce. Both ciphers
/// encrypt by XORing the plaintext with a keystream that depends only on the key and nonce, so
/// the same nonce means the same keystream, and it cancels out: c1 ^ c2 = p1 ^ p2. No key is
/// needed for this step.
fn recover_plaintext_xor(c1: &[u8], c2: &[u8]) -> Vec<u8> {
    xor(c1, c2)
}

fn main() {
    let p1: &[u8] = b"Transfer $100 to account 1111";
    let p2: &[u8] = b"Meet at the north gate at 6pm";

    for alg in [Algorithm::Aes256Gcm, Algorithm::ChaCha20Poly1305] {
        let key = generate_key().unwrap();
        let c1 = broken_encrypt(alg, &key, p1);
        let c2 = broken_encrypt(alg, &key, p2);

        let leaked = recover_plaintext_xor(&c1, &c2);
        assert_eq!(leaked, xor(p1, p2));

        // If the attacker knows or guesses one message (a fixed header, a form letter), the
        // XOR hands them the other one outright.
        let p2_recovered = xor(&leaked, p1);
        assert_eq!(p2_recovered, p2);

        println!("{alg:?} with a reused nonce:");
        println!("  c1 ^ c2       = {}", hex::encode(&leaked));
        println!("  p1 ^ p2       = {}", hex::encode(xor(p1, p2)));
        println!(
            "  p2 from p1    = {:?}",
            String::from_utf8_lossy(&p2_recovered)
        );
    }
    println!("The tags leak too: see \"How it breaks\" in apps/aead/README.md.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reused_nonce_leaks_plaintext_xor() {
        for alg in [Algorithm::Aes256Gcm, Algorithm::ChaCha20Poly1305] {
            let key = generate_key().unwrap();
            // Lengths that cross a 16-byte block boundary, to show the leak is not limited to
            // the first block.
            let p1: Vec<u8> = (0..40).collect();
            let p2: Vec<u8> = (0..40).map(|i| 0xa5 ^ i).collect();
            let c1 = broken_encrypt(alg, &key, &p1);
            let c2 = broken_encrypt(alg, &key, &p2);
            assert_ne!(c1, p1, "{alg:?}: ciphertext must not equal plaintext");
            assert_eq!(recover_plaintext_xor(&c1, &c2), xor(&p1, &p2), "{alg:?}");
        }
    }
}
