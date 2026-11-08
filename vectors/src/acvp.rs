//! Types for the `internalProjection.json` files in `acvp/`, one pair per ACVP vector set.
//!
//! Field names follow the ACVP specifications for each algorithm (usnistgov/ACVP, `src/`).
//! Lengths are in bits, as in the files.

use serde::Deserialize;

use crate::{Bytes, TestFile};

/// `ACVP-AES-GCM-1.0`: AES-GCM per NIST SP 800-38D.
pub type AesGcmFile = TestFile<AesGcmGroup, AesGcmTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AesGcmGroup {
    pub tg_id: u32,
    pub direction: Direction,
    pub key_len: usize,
    pub iv_len: usize,
    pub tag_len: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Encrypt,
    Decrypt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AesGcmTest {
    pub tc_id: u32,
    pub key: Bytes,
    pub iv: Bytes,
    pub aad: Bytes,
    pub pt: Bytes,
    pub ct: Bytes,
    pub tag: Bytes,
    /// False only for decrypt cases whose tag must be rejected.
    pub test_passed: bool,
}

/// `ML-KEM-keyGen-FIPS203` and `ML-DSA-keyGen-FIPS204`: groups differ only by parameter set.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyGenGroup {
    pub tg_id: u32,
    /// e.g. `ML-KEM-768` or `ML-DSA-65`.
    pub parameter_set: String,
}

/// `ML-KEM-keyGen-FIPS203`: FIPS 203 `ML-KEM.KeyGen_internal(d, z)`.
pub type MlKemKeyGenFile = TestFile<KeyGenGroup, MlKemKeyGenTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemKeyGenTest {
    pub tc_id: u32,
    pub d: Bytes,
    pub z: Bytes,
    pub ek: Bytes,
    pub dk: Bytes,
}

/// `ML-KEM-encapDecap-FIPS203`: encapsulation, decapsulation and the FIPS 203 key checks.
pub type MlKemEncapDecapFile = TestFile<MlKemEncapDecapGroup, MlKemEncapDecapTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemEncapDecapGroup {
    pub tg_id: u32,
    pub parameter_set: String,
    pub function: MlKemFunction,
}

/// Which operation a group tests. It decides which optional test fields are set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MlKemFunction {
    /// `ML-KEM.Encaps_internal(ek, m)` gives `c` and `k`.
    Encapsulation,
    /// `ML-KEM.Decaps(dk, c)` gives `k`. There is no pass/fail: for a modified ciphertext, `k`
    /// is the implicit-rejection key, so the expected `k` must still match.
    Decapsulation,
    /// FIPS 203 section 7.2 input check on `ek`; `test_passed` says whether it passes.
    EncapsulationKeyCheck,
    /// FIPS 203 section 7.3 input check on `dk`; `test_passed` says whether it passes.
    DecapsulationKeyCheck,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlKemEncapDecapTest {
    pub tc_id: u32,
    pub ek: Bytes,
    pub dk: Bytes,
    /// Encapsulation randomness. Encapsulation only.
    pub m: Option<Bytes>,
    /// Ciphertext. Encapsulation and decapsulation.
    pub c: Option<Bytes>,
    /// Shared secret. Encapsulation and decapsulation.
    pub k: Option<Bytes>,
    /// Key checks only.
    pub test_passed: Option<bool>,
    /// What was modified, for the decapsulation and key check cases.
    pub reason: Option<String>,
}

/// `ML-DSA-keyGen-FIPS204`: FIPS 204 `ML-DSA.KeyGen_internal(seed)`.
pub type MlDsaKeyGenFile = TestFile<KeyGenGroup, MlDsaKeyGenTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlDsaKeyGenTest {
    pub tc_id: u32,
    pub seed: Bytes,
    pub pk: Bytes,
    pub sk: Bytes,
}

/// Whether tests call the external functions of FIPS 204 (`ML-DSA.Sign`, `HashML-DSA.Sign`
/// and their `Verify`) or the internal ones (`ML-DSA.Sign_internal`, `Verify_internal`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SignatureInterface {
    External,
    Internal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreHash {
    /// `ML-DSA`: the message is signed as is.
    Pure,
    /// `HashML-DSA`: the message is hashed with the test's `hash_alg` first.
    PreHash,
    /// Internal interface, where pre-hashing doesn't apply.
    None,
}

/// `ML-DSA-sigGen-FIPS204`: FIPS 204 signing.
pub type MlDsaSigGenFile = TestFile<MlDsaSigGenGroup, MlDsaSigGenTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlDsaSigGenGroup {
    pub tg_id: u32,
    pub parameter_set: String,
    /// Deterministic signing uses an all-zero `rnd`; otherwise the test gives `rnd`.
    pub deterministic: bool,
    pub signature_interface: SignatureInterface,
    pub pre_hash: PreHash,
    /// The test gives `mu` (the message representative) instead of `message`.
    pub external_mu: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlDsaSigGenTest {
    pub tc_id: u32,
    pub pk: Bytes,
    pub sk: Bytes,
    /// Set unless the group uses `external_mu`.
    pub message: Option<Bytes>,
    /// Set when the group uses `external_mu`.
    pub mu: Option<Bytes>,
    /// Set for the external interface.
    pub context: Option<Bytes>,
    /// Set for hedged (non-deterministic) signing.
    pub rnd: Option<Bytes>,
    /// e.g. `SHA2-256`, or `none` unless the group uses `PreHash::PreHash`.
    pub hash_alg: String,
    pub signature: Bytes,
}

/// `ML-DSA-sigVer-FIPS204`: FIPS 204 verification, including invalid signatures.
pub type MlDsaSigVerFile = TestFile<MlDsaSigVerGroup, MlDsaSigVerTest>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlDsaSigVerGroup {
    pub tg_id: u32,
    pub parameter_set: String,
    pub signature_interface: SignatureInterface,
    pub pre_hash: PreHash,
    pub external_mu: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MlDsaSigVerTest {
    pub tc_id: u32,
    pub pk: Bytes,
    /// Set unless the group uses `external_mu`.
    pub message: Option<Bytes>,
    /// Set when the group uses `external_mu`.
    pub mu: Option<Bytes>,
    /// Set for the external interface.
    pub context: Option<Bytes>,
    pub hash_alg: String,
    pub signature: Bytes,
    /// Whether the signature verifies.
    pub test_passed: bool,
    /// What was modified, or that the signature is valid.
    pub reason: String,
}

/// `SHA2-256-1.0`: SHA-256 per FIPS 180-4.
pub type Sha2File = TestFile<Sha2Group, Sha2Test>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sha2Group {
    pub tg_id: u32,
    pub test_type: Sha2TestType,
}

/// Which kind of test a group holds. It decides which optional test fields are set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Sha2TestType {
    /// Algorithm functional test: hash `msg`, expect `md`.
    Aft,
    /// Monte Carlo test: chained hashing from `msg`. Its results aren't loaded here.
    Mct,
    /// Large data test: an 8 GiB message described by a pattern, not stored. Gives `md` only.
    Ldt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sha2Test {
    pub tc_id: u32,
    /// Message length in bits. Zero for LDT cases.
    pub len: usize,
    /// Set for AFT and MCT.
    pub msg: Option<Bytes>,
    /// Set for AFT and LDT.
    pub md: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Case counts of the files at the ACVP-Server commit pinned in fetch_vectors.py.
    #[test]
    fn every_file_loads_with_all_its_cases() {
        let path = |set| format!("acvp/{set}/internalProjection.json");
        assert_eq!(
            AesGcmFile::load(&path("ACVP-AES-GCM-1.0")).cases().count(),
            60
        );
        let kem_keygen = MlKemKeyGenFile::load(&path("ML-KEM-keyGen-FIPS203"));
        assert_eq!(kem_keygen.cases().count(), 75);
        let encap_decap = MlKemEncapDecapFile::load(&path("ML-KEM-encapDecap-FIPS203"));
        assert_eq!(encap_decap.cases().count(), 165);
        assert_eq!(
            MlDsaKeyGenFile::load(&path("ML-DSA-keyGen-FIPS204"))
                .cases()
                .count(),
            75
        );
        assert_eq!(
            MlDsaSigGenFile::load(&path("ML-DSA-sigGen-FIPS204"))
                .cases()
                .count(),
            360
        );
        assert_eq!(
            MlDsaSigVerFile::load(&path("ML-DSA-sigVer-FIPS204"))
                .cases()
                .count(),
            180
        );
        assert_eq!(Sha2File::load(&path("SHA2-256-1.0")).cases().count(), 517);
    }

    #[test]
    fn aes_gcm_rejections_are_decrypt_cases() {
        let file = AesGcmFile::load("acvp/ACVP-AES-GCM-1.0/internalProjection.json");
        let rejected: Vec<_> = file.cases().filter(|(_, test)| !test.test_passed).collect();
        assert!(!rejected.is_empty());
        assert!(
            rejected
                .iter()
                .all(|(group, _)| group.direction == Direction::Decrypt)
        );
    }

    // The optional fields documented on MlKemEncapDecapTest are set exactly as described.
    #[test]
    fn ml_kem_fields_match_the_function() {
        let file =
            MlKemEncapDecapFile::load("acvp/ML-KEM-encapDecap-FIPS203/internalProjection.json");
        for (group, test) in file.cases() {
            let fields = (test.m.is_some(), test.c.is_some(), test.k.is_some());
            let expected = match group.function {
                MlKemFunction::Encapsulation => (true, true, true),
                MlKemFunction::Decapsulation => (false, true, true),
                _ => (false, false, false),
            };
            assert_eq!(fields, expected, "tcId {}", test.tc_id);
            let key_check = expected == (false, false, false);
            assert_eq!(test.test_passed.is_some(), key_check, "tcId {}", test.tc_id);
        }
    }

    #[test]
    fn ml_dsa_message_or_mu_follows_the_group() {
        let file = MlDsaSigVerFile::load("acvp/ML-DSA-sigVer-FIPS204/internalProjection.json");
        for (group, test) in file.cases() {
            assert_eq!(test.mu.is_some(), group.external_mu, "tcId {}", test.tc_id);
            assert_eq!(
                test.message.is_some(),
                !group.external_mu,
                "tcId {}",
                test.tc_id
            );
            let external = group.signature_interface == SignatureInterface::External;
            assert_eq!(test.context.is_some(), external, "tcId {}", test.tc_id);
        }
        assert!(file.cases().any(|(_, test)| !test.test_passed));
    }

    // The optional fields documented on Sha2Test are set exactly as described, and AFT messages
    // are whole bytes, so a byte-oriented hash API can run them all.
    #[test]
    fn sha2_fields_match_the_test_type() {
        let file = Sha2File::load("acvp/SHA2-256-1.0/internalProjection.json");
        for (group, test) in file.cases() {
            let fields = (test.msg.is_some(), test.md.is_some());
            let expected = match group.test_type {
                Sha2TestType::Aft => (true, true),
                Sha2TestType::Mct => (true, false),
                Sha2TestType::Ldt => (false, true),
            };
            assert_eq!(fields, expected, "tcId {}", test.tc_id);
            if let Some(msg) = &test.msg {
                assert_eq!(msg.len() * 8, test.len, "tcId {}", test.tc_id);
            }
        }
    }
}
