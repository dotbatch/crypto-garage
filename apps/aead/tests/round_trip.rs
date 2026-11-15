use aead_demo::{Algorithm, Error, generate_key, open, seal};

const ALGORITHMS: [Algorithm; 2] = [Algorithm::Aes256Gcm, Algorithm::ChaCha20Poly1305];

#[test]
fn round_trip_recovers_plaintext() {
    for alg in ALGORITHMS {
        let key = generate_key().unwrap();
        let sealed = seal(alg, &key, b"header", b"attack at dawn").unwrap();
        assert_eq!(
            sealed.ciphertext.len(),
            b"attack at dawn".len() + 16,
            "{alg:?}"
        );
        let plain = open(alg, &key, &sealed.nonce, b"header", &sealed.ciphertext).unwrap();
        assert_eq!(plain, b"attack at dawn", "{alg:?}");
    }
}

#[test]
fn empty_plaintext_round_trips() {
    for alg in ALGORITHMS {
        let key = generate_key().unwrap();
        let sealed = seal(alg, &key, &[], &[]).unwrap();
        assert_eq!(sealed.ciphertext.len(), 16, "{alg:?}");
        assert!(
            open(alg, &key, &sealed.nonce, &[], &sealed.ciphertext)
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn each_seal_draws_a_new_nonce() {
    let key = generate_key().unwrap();
    for alg in ALGORITHMS {
        let a = seal(alg, &key, &[], b"same").unwrap();
        let b = seal(alg, &key, &[], b"same").unwrap();
        assert_ne!(a.nonce, b.nonce, "{alg:?}");
        assert_ne!(a.ciphertext, b.ciphertext, "{alg:?}");
    }
}

#[test]
fn tampered_ciphertext_is_rejected() {
    for alg in ALGORITHMS {
        let key = generate_key().unwrap();
        let sealed = seal(alg, &key, b"aad", b"message").unwrap();
        // Flip one bit in the body and one in the tag.
        for index in [0, sealed.ciphertext.len() - 1] {
            let mut bad = sealed.ciphertext.clone();
            bad[index] ^= 0x01;
            assert_eq!(
                open(alg, &key, &sealed.nonce, b"aad", &bad),
                Err(Error::Open),
                "{alg:?}"
            );
        }
    }
}

#[test]
fn wrong_key_nonce_or_aad_is_rejected() {
    for alg in ALGORITHMS {
        let key = generate_key().unwrap();
        let other_key = generate_key().unwrap();
        let sealed = seal(alg, &key, b"aad", b"message").unwrap();
        let mut other_nonce = sealed.nonce;
        other_nonce[0] ^= 0x01;

        let wrong_key = open(alg, &other_key, &sealed.nonce, b"aad", &sealed.ciphertext);
        assert_eq!(wrong_key, Err(Error::Open), "{alg:?} wrong key");
        let wrong_nonce = open(alg, &key, &other_nonce, b"aad", &sealed.ciphertext);
        assert_eq!(wrong_nonce, Err(Error::Open), "{alg:?} wrong nonce");
        let wrong_aad = open(alg, &key, &sealed.nonce, b"other", &sealed.ciphertext);
        assert_eq!(wrong_aad, Err(Error::Open), "{alg:?} wrong aad");
    }
}
