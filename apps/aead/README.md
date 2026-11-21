# aead

Encrypts and authenticates messages with AES-256-GCM or ChaCha20-Poly1305, with random nonces.

## Goal

The first half of the AEAD playground: a small crate that shows the whole seal and open cycle
(key, random nonce, associated data, tag check) with the nonce choice written out in the code.
The browser page comes in a later item.

## Standard

| Primitive | Standard | Library | Vectors |
|---|---|---|---|
| AES-256-GCM | NIST SP 800-38D (November 2007) | `aes-gcm` 0.11.1 (RustCrypto) | Wycheproof `aes_gcm_test.json` |
| ChaCha20-Poly1305 | RFC 8439 (June 2018) | `chacha20poly1305` 0.11.0 (RustCrypto) | Wycheproof `chacha20_poly1305_test.json`, RFC 8439 section 2.8.2 and appendix A.5 |
| Randomness | OS CSPRNG via `getrandom` 0.4.3 | `getrandom` 0.4.3 | n/a |

Both ciphers use 256-bit keys and 96-bit nonces. The tag is 16 bytes and is appended to the
ciphertext.

The known-answer tests in `src/known_answers.rs` run every Wycheproof case with these sizes:
39 valid and 27 invalid for AES-256-GCM, and 256 valid and 60 invalid for ChaCha20-Poly1305. A
valid case must encrypt to the expected ciphertext and tag and decrypt back to the message. An
invalid case must fail to decrypt. Cases with other key, nonce or tag sizes are skipped, because
the API can't express them. The tests are inside the crate because encryption with the vectors'
fixed nonces goes through a private function, so callers only ever get random nonces.

The ACVP `ACVP-AES-GCM-1.0` sample set in `vectors/` has only 128-bit keys, so it has no
AES-256-GCM cases yet.

## Threat model

- Assets: the message, which must stay confidential, and the message's integrity and binding to
  its associated data.
- Attacker: can read and modify ciphertexts in transit, and can replay or swap them.
- In scope: a modified ciphertext, nonce or associated data never decrypts. `open` returns an
  error and no plaintext.
- Out of scope: key storage and distribution, key rotation, side channels, and a compromised
  host. Nonce reuse is a failure mode, not a defense; see the note below.

## How to run

From the repo root:

```sh
cargo test -p aead-demo
cargo clippy -p aead-demo --all-targets -- -D warnings
```

## How it breaks

- Nonces are random 96-bit values from the OS CSPRNG, one per message. Under one key, random
  nonces collide with probability about 2^-33 after 2^32 messages. Past that point, rotate the
  key. NIST SP 800-38D section 8.3 gives the limit for GCM; ChaCha20-Poly1305 has the same
  random-nonce limit in this crate.
- Reusing a nonce under one key breaks GCM: it leaks the XOR of two plaintexts and the
  authentication key. The app never reuses a nonce, so this is shown only in the nonce-reuse
  demo (a later item, clearly labelled).
- `tests/tamper.rs` flips every single bit of the ciphertext, tag, nonce and associated data,
  one at a time, for message and associated-data lengths of 0, 1, 15, 16 and 17 bytes. Every
  flip must return `Error::Open` and no plaintext. `wrong_key_nonce_or_aad_is_rejected` in
  `tests/round_trip.rs` also checks a wrong key.

## Limitations

- Not audited. The crate is a thin wrapper; the ciphers are RustCrypto's.
- The caller owns the key after `generate_key`. The key is zeroized when the returned value is
  dropped, but copies made by the caller are not.
- The implementation does not promise constant-time behavior beyond what RustCrypto provides on
  each target.
- No key derivation, no key wrapping, no streaming. A production version needs key management,
  a nonce counter or XChaCha20-Poly1305 for high-volume use, and a message-length cap per key.
