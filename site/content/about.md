+++
title = "About"
description = "How crypto-garage is built: its principles, where the demos run, and where the material comes from."
+++

crypto-garage is a set of hands-on apps that show modern cryptography working, from the primitives
that run the internet today to the post-quantum algorithms standardized in the last few years. The
code is open on [GitHub](https://github.com/dotbatch/crypto-garage).

## Principles

1. **Tested over clever.** Apps presented as usable call vetted libraries such as RustCrypto,
   libsodium, AWS-LC, OpenSSL and liboqs. Nothing in them is a home-grown primitive.
2. **From-scratch code is labeled as such.** Teaching implementations are kept apart from the apps
   and marked "not for production".
3. **Every primitive is checked against known answers.** NIST ACVP and CAVP vectors, RFC test
   vectors and [Project Wycheproof](https://github.com/C2SP/wycheproof) edge cases run in CI on
   every change.
4. **Cite the standard.** Each project names the FIPS, SP, RFC or ISO document it follows, with the
   version.
5. **Show the failure modes too.** Nonce reuse, missing authentication, timing leaks and downgrade
   attacks get their own demos, labeled as deliberately broken, so the "why" is visible.

## Your input stays in your browser

The demos run in your browser as Rust compiled to WebAssembly. Text and keys you type into a demo
are processed on the page and never sent anywhere: the site is static files, with no server-side
code, no analytics and no scripts, fonts or stylesheets from other sites. Keys a demo generates
come from your browser's cryptographically secure random number generator.

## Public sources only

Everything here is built from public standards, published test vectors and open-source libraries.
It contains nothing from any employer.
