//! Known-answer tests for both AEADs: Wycheproof and RFC 8439.
//!
//! These live inside the crate because the encrypt direction needs the vectors' fixed nonces,
//! and `seal_with_nonce` is private so that callers can only get random ones.

use vectors::wycheproof::{AeadFile, Expected};

use super::{Algorithm, Error, KEY_LEN, NONCE_LEN, open, seal_with_nonce};

/// Checks one case both ways: a valid case must encrypt to exactly `ct || tag` and decrypt back
/// to `msg`; any other case must fail to decrypt and return no plaintext.
fn check(
    alg: Algorithm,
    key: &[u8; KEY_LEN],
    nonce: &[u8; NONCE_LEN],
    aad: &[u8],
    msg: &[u8],
    sealed: &[u8],
    valid: bool,
) -> Result<(), String> {
    let opened = open(alg, key, nonce, aad, sealed);
    if !valid {
        return match opened {
            Err(Error::Open) => Ok(()),
            other => Err(format!("invalid case was not rejected: {other:?}")),
        };
    }
    if seal_with_nonce(alg, key, nonce, aad, msg).as_deref() != Ok(sealed) {
        return Err("wrong ciphertext or tag".into());
    }
    if opened.as_deref() != Ok(msg) {
        return Err(format!("decryption failed: {opened:?}"));
    }
    Ok(())
}

/// Runs every case with a 256-bit key, 96-bit nonce and 128-bit tag, the only sizes this
/// crate's API accepts. Returns how many valid and invalid cases ran.
fn wycheproof(alg: Algorithm, path: &str) -> (usize, usize) {
    let file = AeadFile::load(path);
    let (mut valid, mut invalid) = (0, 0);
    for (group, test) in file.cases() {
        if (group.key_size, group.iv_size, group.tag_size) != (256, 96, 128) {
            continue;
        }
        let is_valid = match test.result {
            Expected::Valid => true,
            Expected::Invalid => false,
            Expected::Acceptable => {
                panic!("tcId {}: no acceptable AEAD cases expected", test.tc_id)
            }
        };
        let key = test.key[..].try_into().unwrap();
        let nonce = test.iv[..].try_into().unwrap();
        let sealed = [&test.ct[..], &test.tag[..]].concat();
        check(alg, key, nonce, &test.aad, &test.msg, &sealed, is_valid)
            .unwrap_or_else(|e| panic!("tcId {} ({}): {e}", test.tc_id, test.comment));
        if is_valid { valid += 1 } else { invalid += 1 }
    }
    (valid, invalid)
}

// The counts pin the cases of the vector files at the commit in fetch_vectors.py, so a loader
// or filter change that silently skips cases fails here. The other cases use 128- or 192-bit
// keys, other nonce sizes or truncated tags, which the API can't express.
#[test]
fn wycheproof_aes_256_gcm() {
    let ran = wycheproof(Algorithm::Aes256Gcm, "wycheproof/aes_gcm_test.json");
    assert_eq!(ran, (39, 27));
}

#[test]
fn wycheproof_chacha20_poly1305() {
    let ran = wycheproof(
        Algorithm::ChaCha20Poly1305,
        "wycheproof/chacha20_poly1305_test.json",
    );
    assert_eq!(ran, (256, 60));
}

fn hex(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap()
}

// RFC 8439 section 2.8.2. The nonce is the 32-bit constant 07000000 followed by the 64-bit IV
// 4041424344454647.
#[test]
fn rfc_8439_section_2_8_2() {
    let key = hex("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
    let nonce = hex("070000004041424344454647");
    let aad = hex("50515253c0c1c2c3c4c5c6c7");
    let msg = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for \
                the future, sunscreen would be it.";
    let sealed = hex(concat!(
        "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d6",
        "3dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b36",
        "92ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc",
        "3ff4def08e4b7a9de576d26586cec64b6116",
        // Tag
        "1ae10b594f09e26a7e902ecbd0600691",
    ));
    let (key, nonce) = (key[..].try_into().unwrap(), nonce[..].try_into().unwrap());
    check(
        Algorithm::ChaCha20Poly1305,
        key,
        nonce,
        &aad,
        msg,
        &sealed,
        true,
    )
    .unwrap();
}

// RFC 8439 appendix A.5: decrypting a received message.
#[test]
fn rfc_8439_appendix_a_5() {
    let key = hex("1c9240a5eb55d38af333888604f6b5f0473917c1402b80099dca5cbc207075c0");
    let nonce = hex("000000000102030405060708");
    let aad = hex("f33388860000000000004e91");
    let msg = "Internet-Drafts are draft documents valid for a maximum of six months and may be \
               updated, replaced, or obsoleted by other documents at any time. It is \
               inappropriate to use Internet-Drafts as reference material or to cite them other \
               than as /\u{201c}work in progress./\u{201d}";
    let sealed = hex(concat!(
        "64a0861575861af460f062c79be643bd5e805cfd345cf389f108670ac76c8cb2",
        "4c6cfc18755d43eea09ee94e382d26b0bdb7b73c321b0100d4f03b7f355894cf",
        "332f830e710b97ce98c8a84abd0b948114ad176e008d33bd60f982b1ff37c855",
        "9797a06ef4f0ef61c186324e2b3506383606907b6a7c02b0f9f6157b53c867e4",
        "b9166c767b804d46a59b5216cde7a4e99040c5a40433225ee282a1b0a06c523e",
        "af4534d7f83fa1155b0047718cbc546a0d072b04b3564eea1b422273f548271a",
        "0bb2316053fa76991955ebd63159434ecebb4e466dae5a1073a6727627097a10",
        "49e617d91d361094fa68f0ff77987130305beaba2eda04df997b714d6c6f2c29",
        "a6ad5cb4022b02709b",
        // Tag
        "eead9d67890cbb22392336fea1851f38",
    ));
    let (key, nonce) = (key[..].try_into().unwrap(), nonce[..].try_into().unwrap());
    let alg = Algorithm::ChaCha20Poly1305;
    check(alg, key, nonce, &aad, msg.as_bytes(), &sealed, true).unwrap();
}
