//! Types for the files in `wycheproof/`, one pair per Wycheproof schema.
//!
//! The schemas are in the `schemas/` directory of C2SP/wycheproof at the commit pinned in
//! `fetch_vectors.py`. Sizes are in bits, as in the files.

use serde::Deserialize;

use crate::{Bytes, TestFile};

/// What a correct implementation does with a test case.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Expected {
    Valid,
    Invalid,
    /// Either outcome is allowed, e.g. for a small-order public key; `flags` says why.
    Acceptable,
}

/// `aead_test_schema_v1`: `aes_gcm_test.json`, `chacha20_poly1305_test.json`.
pub type AeadFile = TestFile<AeadGroup, AeadTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AeadGroup {
    pub key_size: usize,
    pub iv_size: usize,
    pub tag_size: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AeadTest {
    pub tc_id: u32,
    pub comment: String,
    pub flags: Vec<String>,
    pub key: Bytes,
    pub iv: Bytes,
    pub aad: Bytes,
    pub msg: Bytes,
    pub ct: Bytes,
    pub tag: Bytes,
    pub result: Expected,
}

/// `mac_test_schema_v1`: `hmac_sha256_test.json`.
pub type MacFile = TestFile<MacGroup, MacTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacGroup {
    pub key_size: usize,
    /// When smaller than the MAC's output, the expected tags are truncated to this size.
    pub tag_size: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacTest {
    pub tc_id: u32,
    pub comment: String,
    pub flags: Vec<String>,
    pub key: Bytes,
    pub msg: Bytes,
    pub tag: Bytes,
    pub result: Expected,
}

/// `hkdf_test_schema_v1`: `hkdf_sha256_test.json`.
pub type HkdfFile = TestFile<HkdfGroup, HkdfTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HkdfGroup {
    pub key_size: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HkdfTest {
    pub tc_id: u32,
    pub comment: String,
    pub flags: Vec<String>,
    pub ikm: Bytes,
    pub salt: Bytes,
    pub info: Bytes,
    /// Requested output length in bytes. Invalid cases ask for more than HKDF allows.
    pub size: usize,
    pub okm: Bytes,
    pub result: Expected,
}

/// `xdh_comp_schema_v1`: `x25519_test.json`.
pub type XdhFile = TestFile<XdhGroup, XdhTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdhGroup {
    pub curve: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdhTest {
    pub tc_id: u32,
    pub comment: String,
    pub flags: Vec<String>,
    pub public: Bytes,
    pub private: Bytes,
    pub shared: Bytes,
    pub result: Expected,
}

/// `eddsa_verify_schema_v1`: `ed25519_test.json`.
pub type EddsaVerifyFile = TestFile<EddsaVerifyGroup, SignatureTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EddsaVerifyGroup {
    pub public_key: EddsaPublicKey,
}

#[derive(Debug, Deserialize)]
pub struct EddsaPublicKey {
    /// The raw 32-byte public key.
    pub pk: Bytes,
}

/// `ecdsa_verify_schema_v1`: `ecdsa_secp256r1_sha256_test.json`.
pub type EcdsaVerifyFile = TestFile<EcdsaVerifyGroup, SignatureTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EcdsaVerifyGroup {
    pub public_key: EcPublicKey,
    /// The hash applied to `msg` before signing, e.g. `SHA-256`.
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct EcPublicKey {
    pub curve: String,
    /// SEC 1 uncompressed point: `04 || x || y`.
    pub uncompressed: Bytes,
}

/// A signature verification case, shared by the EdDSA and ECDSA schemas.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureTest {
    pub tc_id: u32,
    pub comment: String,
    pub flags: Vec<String>,
    pub msg: Bytes,
    /// For ECDSA, DER-encoded. Many invalid cases are malformed encodings.
    pub sig: Bytes,
    pub result: Expected,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each count is the file's own `numberOfTests`, so a schema mismatch that drops cases fails.
    #[test]
    fn every_file_loads_with_all_its_cases() {
        let path = |name| format!("wycheproof/{name}");
        assert_eq!(
            AeadFile::load(&path("aes_gcm_test.json")).cases().count(),
            316
        );
        assert_eq!(
            AeadFile::load(&path("chacha20_poly1305_test.json"))
                .cases()
                .count(),
            325
        );
        assert_eq!(
            MacFile::load(&path("hmac_sha256_test.json"))
                .cases()
                .count(),
            174
        );
        assert_eq!(
            HkdfFile::load(&path("hkdf_sha256_test.json"))
                .cases()
                .count(),
            86
        );
        assert_eq!(
            XdhFile::load(&path("x25519_test.json")).cases().count(),
            518
        );
        assert_eq!(
            EddsaVerifyFile::load(&path("ed25519_test.json"))
                .cases()
                .count(),
            210
        );
        let ecdsa = EcdsaVerifyFile::load(&path("ecdsa_secp256r1_sha256_test.json"));
        assert_eq!(ecdsa.cases().count(), 484);
    }

    #[test]
    fn first_aes_gcm_case_matches_the_file() {
        let file = AeadFile::load("wycheproof/aes_gcm_test.json");
        let (group, test) = file.cases().next().unwrap();
        assert_eq!(file.algorithm, "AES-GCM");
        assert_eq!(
            (group.key_size, group.iv_size, group.tag_size),
            (128, 96, 128)
        );
        assert_eq!(test.tc_id, 1);
        assert_eq!(
            test.key.0,
            hex::decode("5b9604fe14eadba931b0ccf34843dab9").unwrap()
        );
        assert!(test.aad.is_empty());
        assert_eq!(test.result, Expected::Valid);
    }

    #[test]
    fn all_three_verdicts_are_read() {
        let file = EddsaVerifyFile::load("wycheproof/ed25519_test.json");
        for expected in [Expected::Valid, Expected::Invalid, Expected::Acceptable] {
            assert!(
                file.cases().any(|(_, test)| test.result == expected),
                "{expected:?}"
            );
        }
    }
}
