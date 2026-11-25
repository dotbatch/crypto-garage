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

The nonce-reuse failure demo:

```sh
cargo run -p aead-demo --example nonce_reuse
```

## How it breaks

- Nonces are random 96-bit values from the OS CSPRNG, one per message. Under one key, random
  nonces collide with probability about 2^-33 after 2^32 messages. Past that point, rotate the
  key. NIST SP 800-38D section 8.3 gives the limit for GCM; ChaCha20-Poly1305 has the same
  random-nonce limit in this crate.
- Reusing a nonce under one key breaks both ciphers. The crate's `seal` never lets a caller
  choose the nonce, so this is shown only in the nonce-reuse demo below.
- `tests/tamper.rs` flips every single bit of the ciphertext, tag, nonce and associated data,
  one at a time, for message and associated-data lengths of 0, 1, 15, 16 and 17 bytes. Every
  flip must return `Error::Open` and no plaintext. `wrong_key_nonce_or_aad_is_rejected` in
  `tests/round_trip.rs` also checks a wrong key.

### Nonce-reuse demo (deliberately broken)

`examples/nonce_reuse.rs` is **deliberately broken**. It encrypts two messages under one key
with one fixed nonce, which both standards forbid, and nothing outside the file imports it.
Its test runs under `cargo test`.

**The plaintexts leak.** Both ciphers encrypt by XORing the message with a keystream that
depends only on the key and the nonce: AES in counter mode for GCM, the ChaCha20 block function
for ChaCha20-Poly1305. The same key and nonce give the same keystream, so for two ciphertexts
`c1 ^ c2 = p1 ^ p2`. The key cancels out. The demo checks this for both ciphers, then shows that
anyone who knows or guesses one message (a fixed header, a form letter) reads the other one.

**The GCM authentication key leaks: the forbidden attack.** GCM's tag is
`T = GHASH_H(A, C) ^ E_K(J0)`. `H = E_K(0^128)` is the hash key, and it is the same for every
message under the key. `E_K(J0)` depends on the nonce, so it repeats when the nonce repeats.
XORing two tags made with the same nonce cancels `E_K(J0)` and leaves
`T1 ^ T2 = GHASH_H(A1, C1) ^ GHASH_H(A2, C2)`. That is a polynomial in `H` over GF(2^128), and
every coefficient is a block the attacker has already seen. Finding its roots gives a few
candidates for `H`. With `H`, the attacker gets `E_K(J0)` for any nonce they have seen one valid
message under (`E_K(J0) = T ^ GHASH_H(A, C)`), and can then forge valid tags for any ciphertext
and associated data under that nonce. Joux described the attack in comments to NIST on the GCM
draft (2006). Böck, Zauner, Devlin, Somorovsky and Jovanovic found it against real TLS servers
("Nonce-Disrespecting Adversaries", USENIX WOOT 2016).

**ChaCha20-Poly1305 leaks less, but still allows forgery.** The Poly1305 key `(r, s)` comes from
the ChaCha20 keystream for that nonce (RFC 8439 section 2.6), so it repeats when the nonce
repeats. Two tags under the same nonce give a few candidate equations in `r` modulo 2^130 - 5
(the tag drops the bits above 2^128), and solving them lets the attacker forge messages under
that nonce. Unlike GCM's `H`, `(r, s)` is fresh for each nonce, so the damage stays with the
reused nonce rather than spreading to the whole key.

The demo recovers only the plaintext XOR. The tag attacks need polynomial root finding, and
this repo leaves that math to `learn/`. The fix for all of this is never to reuse a nonce:
random 96-bit nonces with key rotation (as `seal` does), a counter, or XChaCha20-Poly1305, whose
192-bit nonces are safe to draw at random.

## Limitations

- Not audited. The crate is a thin wrapper; the ciphers are RustCrypto's.
- The caller owns the key after `generate_key`. The key is zeroized when the returned value is
  dropped, but copies made by the caller are not.
- The implementation does not promise constant-time behavior beyond what RustCrypto provides on
  each target.
- No key derivation, no key wrapping, no streaming. A production version needs key management,
  a nonce counter or XChaCha20-Poly1305 for high-volume use, and a message-length cap per key.
