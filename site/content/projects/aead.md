+++
title = "AEAD playground"
description = "AES-256-GCM and ChaCha20-Poly1305 side by side, and a live demo of what nonce reuse breaks."
weight = 10
template = "aead.html"

[extra]
standards = ["SP 800-38D", "RFC 8439"]
status = "live"
code = "apps/aead"
+++

The page loads a WebAssembly module built from Rust. When it starts, the module draws a random
256-bit key from your browser's `crypto.getRandomValues` and keeps it in Wasm memory; this page's
script never sees it. Each time you encrypt, the module draws a fresh random 96-bit nonce,
encrypts the message with RustCrypto's AES-256-GCM or ChaCha20-Poly1305, and appends a 16-byte
tag that covers the ciphertext and the associated data. Switching algorithm makes a new key.

Change any byte and decrypt, or edit the associated data after encrypting, and decryption fails
with no plaintext at all. That is the point of authenticated encryption: a modified message is
rejected, not decrypted into something plausible. The same code is checked in CI against the
Wycheproof test vectors for both ciphers and against every single-bit flip of the ciphertext,
tag, nonce and associated data.

The nonce-reuse toggle is deliberately broken. It encrypts two messages under one key and one
nonce, which both standards forbid. Both ciphers then XOR the two messages with the same
keystream, so XORing the two ciphertexts cancels it and leaves the XOR of the two messages,
without the key. Anyone who knows one message reads the other. For GCM it is worse: the two tags
leak the authentication key, which lets an attacker forge messages. The fix is never to reuse a
nonce: random 96-bit nonces with a key change well before 2^32 messages, a counter, or
XChaCha20-Poly1305.
