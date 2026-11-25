//! DELIBERATELY BROKEN: FAILURE-MODE DEMO. DO NOT COPY.
//!
//! This file encrypts two messages under the same key AND the same nonce, which both AES-GCM
//! and ChaCha20-Poly1305 forbid. It backs the page's nonce-reuse toggle, and mirrors
//! `apps/aead/examples/nonce_reuse.rs`. The key is a throwaway made here for the two messages,
//! never a `Playground` key. See "How it breaks" in `apps/aead/README.md`.

use aead_demo::{KEY_LEN, NONCE_LEN, generate_key};
use aes_gcm::Aes256Gcm;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::{Aead, KeyInit};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Algorithm, describe, parse_algorithm};

/// The bug being demonstrated: one fixed nonce for every message.
const REUSED_NONCE: [u8; NONCE_LEN] = [0; NONCE_LEN];

/// Two ciphertexts, without their tags, that shared a key and a nonce.
#[wasm_bindgen]
pub struct ReusedNonce {
    c1: Vec<u8>,
    c2: Vec<u8>,
}

#[wasm_bindgen]
impl ReusedNonce {
    #[wasm_bindgen(getter)]
    pub fn c1(&self) -> Vec<u8> {
        self.c1.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn c2(&self) -> Vec<u8> {
        self.c2.clone()
    }
}

/// Encrypts `p1` and `p2` under one fresh key and the reused nonce. The page then XORs the two
/// ciphertexts, as an eavesdropper would, and gets `p1 ^ p2` without the key.
#[wasm_bindgen]
pub fn seal_reusing_nonce(algorithm: &str, p1: &str, p2: &str) -> Result<ReusedNonce, String> {
    let algorithm = parse_algorithm(algorithm)?;
    let key = generate_key().map_err(describe)?;
    Ok(ReusedNonce {
        c1: broken_encrypt(algorithm, &key, p1.as_bytes()),
        c2: broken_encrypt(algorithm, &key, p2.as_bytes()),
    })
}

/// Encrypts with the reused nonce and drops the tag, which the XOR leak doesn't need.
fn broken_encrypt(algorithm: Algorithm, key: &[u8; KEY_LEN], plaintext: &[u8]) -> Vec<u8> {
    let nonce = REUSED_NONCE.into();
    let mut sealed = match algorithm {
        Algorithm::Aes256Gcm => Aes256Gcm::new(key.into()).encrypt(&nonce, plaintext),
        Algorithm::ChaCha20Poly1305 => ChaCha20Poly1305::new(key.into()).encrypt(&nonce, plaintext),
    }
    // Only fails for messages over 2^36 bytes, far beyond what a text box holds.
    .expect("typed text is within the cipher's length limit");
    sealed.truncate(plaintext.len());
    sealed
}
