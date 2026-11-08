# hello-wasm

Hashes text typed into a web page with SHA-256, in Rust compiled to WebAssembly.

## Goal

The site's first in-browser demo, and the smallest end-to-end test of the Wasm pipeline: a Rust
crate built with `wasm-pack`, loaded by a page with no third-party scripts, and checked against
NIST vectors in CI. Later demos follow the same path.

## Standard

| Primitive | Standard | Library | Vectors |
|---|---|---|---|
| SHA-256 | FIPS 180-4 (August 2015) | `sha2` 0.11 (RustCrypto) | ACVP `SHA2-256-1.0`, AFT cases |

Text is encoded as UTF-8 before hashing.

## Threat model

- Assets: none. The input is text the visitor types, and it never leaves the page.
- Attacker: out of scope for a hash of public text.
- In scope: showing that the browser computes the same digest as any other SHA-256
  implementation.
- Out of scope: hashing secrets. A bare SHA-256 of a password or key is not a safe way to store
  or check it; that needs a password hash (Argon2) or a MAC (HMAC).

## How to run

From the repo root:

```sh
cargo test -p hello-wasm
```

To try the page locally, build the module into the site, then serve the site:

```sh
wasm-pack build apps/hello-wasm --target web --no-pack --out-dir ../../site/static/demos/hello-wasm/pkg
cd site && zola serve
```

The demo is at `http://127.0.0.1:1111/projects/hello-wasm/`.

## How it breaks

A hash has no key to get wrong or tag to forge, so there's no rejection test. What can go wrong is
encoding: hashing UTF-16 code units instead of UTF-8 bytes gives a different digest for any
non-ASCII text. `tests/known_answers.rs` checks that `"é"` hashes as the two bytes `C3 A9`.

## Limitations

- Not audited. The hash itself is RustCrypto's `sha2`; this crate only adapts it for JavaScript.
- The whole input is copied into Wasm memory and hashed at once, which is fine for typed text but
  not for large files. A file hasher would stream chunks through `Sha256::update`.
