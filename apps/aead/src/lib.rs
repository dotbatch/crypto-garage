//! Authenticated encryption with AES-256-GCM (NIST SP 800-38D) and ChaCha20-Poly1305 (RFC 8439).
//!
//! The ciphers come from RustCrypto; this crate chooses the nonce and keeps the key handling in
//! one place. It does not implement any primitive itself.

use aes_gcm::Aes256Gcm;
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use zeroize::Zeroizing;

/// Both algorithms use 256-bit keys and 96-bit nonces.
pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sealed {
    pub nonce: [u8; NONCE_LEN],
    /// Ciphertext followed by the 16-byte authentication tag.
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The OS random number generator failed.
    Random,
    /// The message is too long for the cipher (over 2^36 - 32 bytes).
    Seal,
    /// The tag did not verify: wrong key, nonce, associated data, or a modified message.
    Open,
}

/// A fresh 256-bit key from the OS CSPRNG. Zeroized when dropped.
pub fn generate_key() -> Result<Zeroizing<[u8; KEY_LEN]>, Error> {
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    getrandom::fill(&mut *key).map_err(|_| Error::Random)?;
    Ok(key)
}

/// Encrypts and authenticates `plaintext`, binding it to `aad`.
///
/// Nonce uniqueness: each call draws a fresh random 96-bit nonce from the OS CSPRNG. Under one
/// key, random nonces collide with probability about 2^-33 after 2^32 messages, so a key must
/// be rotated well before that (NIST SP 800-38D, section 8.3). Both ciphers use the same rule.
pub fn seal(
    algorithm: Algorithm,
    key: &[u8; KEY_LEN],
    aad: &[u8],
    plaintext: &[u8],
) -> Result<Sealed, Error> {
    let mut nonce = [0u8; NONCE_LEN];
    getrandom::fill(&mut nonce).map_err(|_| Error::Random)?;
    let payload = Payload {
        msg: plaintext,
        aad,
    };
    let ciphertext = match algorithm {
        Algorithm::Aes256Gcm => Aes256Gcm::new(key.into()).encrypt(&nonce.into(), payload),
        Algorithm::ChaCha20Poly1305 => {
            ChaCha20Poly1305::new(key.into()).encrypt(&nonce.into(), payload)
        }
    }
    .map_err(|_| Error::Seal)?;
    Ok(Sealed { nonce, ciphertext })
}

/// Checks the tag and returns the plaintext. Returns `Error::Open` and no plaintext on any
/// mismatch.
pub fn open(
    algorithm: Algorithm,
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, Error> {
    let payload = Payload {
        msg: ciphertext,
        aad,
    };
    match algorithm {
        Algorithm::Aes256Gcm => Aes256Gcm::new(key.into()).decrypt(&(*nonce).into(), payload),
        Algorithm::ChaCha20Poly1305 => {
            ChaCha20Poly1305::new(key.into()).decrypt(&(*nonce).into(), payload)
        }
    }
    .map_err(|_| Error::Open)
}
