//! Typed loaders for the test vectors in `vectors/`, for use as a dev-dependency.
//!
//! Wycheproof and ACVP files share one shape: a list of test groups, each holding parameters
//! common to its tests plus the tests themselves. [`TestFile::cases`] flattens that into
//! `(group, test)` pairs, so a test runs every case of a file in one loop:
//!
//! ```
//! use vectors::wycheproof::AeadFile;
//!
//! for (group, test) in AeadFile::load("wycheproof/aes_gcm_test.json").cases() {
//!     assert_eq!(test.key.len() * 8, group.key_size, "tcId {}", test.tc_id);
//! }
//! ```
//!
//! Everything loaded here is public test data, never real key material, which is why the types
//! derive `Debug` and failing tests may print them.

pub mod acvp;
pub mod wycheproof;

use std::{fmt, fs, ops::Deref, path::Path};

use serde::{Deserialize, Deserializer, de::DeserializeOwned};

/// A vector file: groups of tests whose parameters are `G` and whose cases are `T`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestFile<G, T> {
    pub algorithm: String,
    pub test_groups: Vec<TestGroup<G, T>>,
}

#[derive(Debug, Deserialize)]
pub struct TestGroup<G, T> {
    /// The group's other fields, read into the schema's group type.
    #[serde(flatten)]
    pub params: G,
    pub tests: Vec<T>,
}

impl<G: DeserializeOwned, T: DeserializeOwned> TestFile<G, T> {
    /// Loads a file by its path relative to `vectors/`, e.g. `"wycheproof/aes_gcm_test.json"`.
    ///
    /// Panics if the file is missing or doesn't match the types, since no test can run then.
    pub fn load(path: &str) -> Self {
        let full = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        let json = fs::read_to_string(&full).unwrap_or_else(|e| {
            panic!(
                "{}: {e} (run `uv run vectors/fetch_vectors.py`)",
                full.display()
            )
        });
        serde_json::from_str(&json).unwrap_or_else(|e| panic!("{}: {e}", full.display()))
    }
}

impl<G, T> TestFile<G, T> {
    /// Every test in the file, paired with the parameters of its group.
    pub fn cases(&self) -> impl Iterator<Item = (&G, &T)> {
        self.test_groups
            .iter()
            .flat_map(|group| group.tests.iter().map(move |test| (&group.params, test)))
    }
}

/// Bytes stored as a hex string: lowercase in Wycheproof, uppercase in ACVP.
#[derive(Clone, PartialEq, Eq)]
pub struct Bytes(pub Vec<u8>);

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hex = String::deserialize(deserializer)?;
        hex::decode(hex)
            .map(Bytes)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_decode_either_case() {
        let lower: Bytes = serde_json::from_str(r#""0aff""#).unwrap();
        let upper: Bytes = serde_json::from_str(r#""0AFF""#).unwrap();
        assert_eq!(*lower, [0x0a, 0xff]);
        assert_eq!(lower, upper);
        let empty: Bytes = serde_json::from_str(r#""""#).unwrap();
        assert!(empty.is_empty());
    }

    #[test]
    #[should_panic(expected = "fetch_vectors.py")]
    fn missing_file_names_the_fetch_script() {
        wycheproof::AeadFile::load("wycheproof/no_such_file.json");
    }

    #[test]
    #[should_panic(expected = "missing field")]
    fn wrong_schema_is_rejected() {
        wycheproof::AeadFile::load("acvp/ACVP-AES-GCM-1.0/internalProjection.json");
    }

    #[test]
    fn bytes_reject_bad_hex() {
        assert!(serde_json::from_str::<Bytes>(r#""abc""#).is_err());
        assert!(serde_json::from_str::<Bytes>(r#""zz""#).is_err());
    }
}
