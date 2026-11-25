# aead-wasm

Runs the AEAD playground in the browser: encrypt a message, tamper with it, watch decryption
fail, and see what nonce reuse leaks.

## Goal

The browser half of the AEAD playground. It compiles [`aead-demo`](../aead/) to WebAssembly so
a visitor can try AES-256-GCM and ChaCha20-Poly1305 without installing anything. The crate is a
thin adapter: the ciphers, the random nonces and the tests that matter are in `apps/aead`.

## Standard

| Primitive | Standard | Library | Vectors |
|---|---|---|---|
| AES-256-GCM | NIST SP 800-38D (November 2007) | `aes-gcm` 0.11.1 via `aead-demo` | Wycheproof, in `apps/aead` |
| ChaCha20-Poly1305 | RFC 8439 (June 2018) | `chacha20poly1305` 0.11.0 via `aead-demo` | Wycheproof and RFC 8439, in `apps/aead` |
| Randomness | Web Crypto `crypto.getRandomValues` | `getrandom` 0.4.3 with `wasm_js` | n/a |

Messages and associated data are encoded as UTF-8. The tag is the last 16 bytes of the
ciphertext.

## Threat model

- Assets: the demo key, which is generated in Wasm memory when the page loads and never handed
  to JavaScript. The page shows nonces, ciphertexts and tags only.
- Attacker: for the demo, the visitor, who can change any byte of the ciphertext, tag or
  associated data before decrypting.
- In scope: a changed byte or changed associated data never decrypts; the page shows the
  rejection and no plaintext.
- Out of scope: protecting real secrets. Anything typed stays in the page, but a browser tab is
  not a place for real keys or messages. Other scripts in the page, browser extensions and side
  channels are out of scope too.

## How to run

From the repo root:

```sh
cargo test -p aead-wasm
cargo clippy -p aead-wasm --target wasm32-unknown-unknown -- -D warnings
```

To try the page locally, build the module into the site, then serve the site:

```sh
wasm-pack build apps/aead-wasm --target web --no-pack --out-dir ../../site/static/demos/aead/pkg
cd site && zola serve
```

The demo is at `http://127.0.0.1:1111/projects/aead/`.

## How it breaks

- `tests/playground.rs` checks the page's paths natively: a flipped byte in the ciphertext or the
  tag, changed associated data, a key from another playground and a short nonce are all
  rejected. The full bit-flip and known-answer tests are in `apps/aead`.
- Each algorithm gets its own key. Switching algorithm makes a new `Playground` and frees the
  old one, which zeroizes its key, so the two ciphers never share a key.
- `src/nonce_reuse.rs` is **deliberately broken**. It encrypts two messages under one key and one
  fixed nonce to back the page's nonce-reuse toggle, and the page then XORs the two ciphertexts
  to get the XOR of the plaintexts without the key. It uses a throwaway key, never a
  `Playground` key, and nothing else in the crate calls it. The explanation, including how the
  GCM authentication key leaks, is in "How it breaks" in the [`aead` README](../aead/README.md).

## Limitations

- Not audited. All cryptography is RustCrypto's, through `aead-demo`.
- The key lives in Wasm linear memory, which page scripts can read. Zeroizing on `free()` limits
  how long it stays there, not who can see it.
- Errors cross into JavaScript as plain strings, which is enough for a demo page but not a
  typed error API.
