//! SHA-256 (FIPS 180-4) of text typed into a web page, computed in WebAssembly.
//!
//! The hash comes from RustCrypto's `sha2`; this crate only adapts it for JavaScript. The input
//! is public text, not a secret, so nothing here needs constant-time handling or zeroizing.

use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::wasm_bindgen;

/// SHA-256 of the UTF-8 bytes of `text`, as 64 lowercase hex characters.
///
/// JavaScript strings are UTF-16; `wasm-bindgen` converts them to UTF-8 on the way in, so the
/// result matches `printf '%s' text | sha256sum` on a UTF-8 system.
#[wasm_bindgen]
pub fn sha256_hex(text: &str) -> String {
    hex::encode(Sha256::digest(text.as_bytes()))
}
