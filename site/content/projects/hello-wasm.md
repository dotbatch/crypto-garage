+++
title = "SHA-256 in the browser"
description = "Type text and see its SHA-256 digest, computed by Rust compiled to WebAssembly."
weight = 100
template = "hello-wasm.html"

[extra]
standards = ["FIPS 180-4"]
status = "live"
code = "apps/hello-wasm"
+++

The page loads a small WebAssembly module built from Rust. Each keystroke passes the text into the
module, which encodes it as UTF-8 and hashes it with RustCrypto's `sha2`, then hands back the
digest as hex. Nothing is sent anywhere: the hashing happens in your browser.

The starting text, `abc`, is the worked example in FIPS 180-4, so the first digest you see is the
one printed in the standard. The same code is checked in CI against all 512 functional test cases
in NIST's ACVP SHA2-256 vector set.

SHA-256 alone is the wrong tool for secrets. A plain hash of a password can be brute-forced
quickly, and a plain hash of a key and message is open to length extension. Use a password hash
such as Argon2, or a MAC such as HMAC-SHA256.
