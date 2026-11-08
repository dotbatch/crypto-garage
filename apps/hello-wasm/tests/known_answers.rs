use hello_wasm::sha256_hex;
use sha2::{Digest, Sha256};
use vectors::acvp::{Sha2File, Sha2TestType};

// Every AFT case of the NIST ACVP SHA2-256 set. MCT and LDT cases test the hash library's
// chaining and 8 GiB inputs, which this demo adds nothing to.
#[test]
fn acvp_sha2_256() {
    let file = Sha2File::load("acvp/SHA2-256-1.0/internalProjection.json");
    let mut ran = 0;
    for (group, test) in file.cases() {
        if group.test_type != Sha2TestType::Aft {
            continue;
        }
        let (msg, md) = (test.msg.as_ref().unwrap(), test.md.as_ref().unwrap());
        assert_eq!(Sha256::digest(&msg[..])[..], md[..], "tcId {}", test.tc_id);
        ran += 1;
    }
    assert_eq!(ran, 512);
}

// The exported function: UTF-8 encoding and hex output, checked against the FIPS 180-4
// "abc" example (NIST CSRC, Examples with Intermediate Values, SHA-256).
#[test]
fn sha256_hex_of_abc() {
    assert_eq!(
        sha256_hex("abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn sha256_hex_of_empty_text() {
    assert_eq!(
        sha256_hex(""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

// Non-ASCII text is hashed as UTF-8: "é" is the two bytes C3 A9.
#[test]
fn sha256_hex_hashes_utf8() {
    assert_eq!(sha256_hex("é"), hex::encode(Sha256::digest([0xc3, 0xa9])));
}
