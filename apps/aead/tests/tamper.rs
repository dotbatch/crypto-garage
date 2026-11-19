//! Negative tests: flipping any single bit of the ciphertext, tag, nonce or associated data
//! must make `open` fail and return no plaintext.
//!
//! Every bit is flipped rather than a random sample. The inputs are small enough to try them
//! all, and a skipped bit could be the one a forgery needs.

use aead_demo::{Algorithm, Error, NONCE_LEN, generate_key, open, seal};

const ALGORITHMS: [Algorithm; 2] = [Algorithm::Aes256Gcm, Algorithm::ChaCha20Poly1305];
const TAG_LEN: usize = 16;

/// Lengths around 16 bytes, the block size of both MACs (GHASH and Poly1305), so empty, partial
/// and full blocks of message and associated data are all covered.
const LENGTHS: [usize; 5] = [0, 1, 15, 16, 17];

fn flip(bytes: &[u8], bit: usize) -> Vec<u8> {
    let mut out = bytes.to_vec();
    out[bit / 8] ^= 1 << (bit % 8);
    out
}

#[test]
fn every_single_bit_flip_is_rejected() {
    for alg in ALGORITHMS {
        let key = generate_key().unwrap();
        for msg_len in LENGTHS {
            for aad_len in LENGTHS {
                let msg: Vec<u8> = (0..msg_len as u8).collect();
                let aad: Vec<u8> = (0..aad_len as u8).map(|i| 0x80 | i).collect();
                let sealed = seal(alg, &key, &aad, &msg).unwrap();
                let (nonce, ct) = (sealed.nonce, &sealed.ciphertext);
                let case = format!("{alg:?}, {msg_len}-byte message, {aad_len}-byte aad");

                // The untouched input must open, so a rejection below is due to the flip alone.
                assert_eq!(open(alg, &key, &nonce, &aad, ct), Ok(msg.clone()), "{case}");
                assert_eq!(ct.len(), msg_len + TAG_LEN, "{case}");

                for bit in 0..ct.len() * 8 {
                    let part = if bit / 8 < msg_len {
                        "ciphertext"
                    } else {
                        "tag"
                    };
                    let opened = open(alg, &key, &nonce, &aad, &flip(ct, bit));
                    assert_eq!(opened, Err(Error::Open), "{case}: {part} bit {bit}");
                }
                for bit in 0..NONCE_LEN * 8 {
                    let bad_nonce = flip(&nonce, bit).try_into().unwrap();
                    let opened = open(alg, &key, &bad_nonce, &aad, ct);
                    assert_eq!(opened, Err(Error::Open), "{case}: nonce bit {bit}");
                }
                for bit in 0..aad_len * 8 {
                    let opened = open(alg, &key, &nonce, &flip(&aad, bit), ct);
                    assert_eq!(opened, Err(Error::Open), "{case}: aad bit {bit}");
                }
            }
        }
    }
}
