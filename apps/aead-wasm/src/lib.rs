//! The AEAD playground's browser demo: `aead-demo`'s AES-256-GCM (NIST SP 800-38D) and
//! ChaCha20-Poly1305 (RFC 8439) exposed to JavaScript.
//!
//! This crate implements no cryptography. It adapts `aead-demo` for the page, and keeps the key
//! inside Wasm memory: JavaScript sees nonces, ciphertexts and tags, never key material.
//!
//! Errors are returned as strings rather than `JsError` so the native tests can check them;
//! `wasm-bindgen` turns them into thrown JavaScript values.

mod nonce_reuse;

pub use nonce_reuse::{ReusedNonce, seal_reusing_nonce};

use aead_demo::{Algorithm, Error, KEY_LEN, NONCE_LEN, Sealed, generate_key, open, seal};
use wasm_bindgen::prelude::wasm_bindgen;
use zeroize::Zeroizing;

/// The names the page uses for the two algorithms.
fn parse_algorithm(name: &str) -> Result<Algorithm, String> {
    match name {
        "aes-256-gcm" => Ok(Algorithm::Aes256Gcm),
        "chacha20-poly1305" => Ok(Algorithm::ChaCha20Poly1305),
        _ => Err(format!("unknown algorithm: {name}")),
    }
}

fn describe(error: Error) -> String {
    match error {
        Error::Random => "the browser's random number generator failed",
        Error::Seal => "the message is too long to encrypt",
        Error::Open => "rejected: the tag does not match, so no plaintext is released",
    }
    .to_string()
}

/// One algorithm and its own random key, held in Wasm memory and zeroized when the page calls
/// `free()`. Switching algorithm on the page makes a new `Playground`, so the two ciphers never
/// share a key.
#[wasm_bindgen]
pub struct Playground {
    algorithm: Algorithm,
    key: Zeroizing<[u8; KEY_LEN]>,
}

#[wasm_bindgen]
impl Playground {
    #[wasm_bindgen(constructor)]
    pub fn new(algorithm: &str) -> Result<Playground, String> {
        Ok(Self {
            algorithm: parse_algorithm(algorithm)?,
            key: generate_key().map_err(describe)?,
        })
    }

    /// Encrypts the UTF-8 bytes of `plaintext`, bound to `aad`. `aead_demo::seal` draws a fresh
    /// random nonce for every call, so no two messages under this key share one.
    pub fn seal(&self, aad: &str, plaintext: &str) -> Result<SealedMessage, String> {
        seal(
            self.algorithm,
            &self.key,
            aad.as_bytes(),
            plaintext.as_bytes(),
        )
        .map(SealedMessage)
        .map_err(describe)
    }

    /// Checks the tag and returns the plaintext bytes, or an error and no plaintext.
    pub fn open(&self, nonce: &[u8], aad: &str, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let nonce: &[u8; NONCE_LEN] = nonce
            .try_into()
            .map_err(|_| format!("the nonce must be {NONCE_LEN} bytes"))?;
        open(self.algorithm, &self.key, nonce, aad.as_bytes(), ciphertext).map_err(describe)
    }
}

/// A sealed message as the page shows it.
#[wasm_bindgen]
pub struct SealedMessage(Sealed);

#[wasm_bindgen]
impl SealedMessage {
    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> Vec<u8> {
        self.0.nonce.to_vec()
    }

    /// The ciphertext followed by the 16-byte tag.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.0.ciphertext.clone()
    }
}
