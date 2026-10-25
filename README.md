# crypto-garage

Hands-on apps that show modern cryptography working, from the proven primitives that run the internet today to the post-quantum algorithms standardized in the last few years. Several projects lean toward **vehicle cybersecurity**, where cryptography has to fit in small ECUs, slow buses and long vehicle lifetimes.

> **Status:** planning. See [ROADMAP.md](ROADMAP.md) for the project list.

## Principles

1. **Tested over clever.** Production-style demos use vetted libraries (libsodium, RustCrypto, AWS-LC, OpenSSL 3.5+, liboqs). No home-grown primitives in anything presented as usable.
2. **From-scratch code is labeled as such.** Teaching implementations live under `learn/` and carry a "not for production" banner.
3. **Every primitive checks against known-answer tests.** NIST ACVP / CAVP vectors, RFC test vectors and [Project Wycheproof](https://github.com/C2SP/wycheproof) edge cases run in CI.
4. **Cite the standard.** Each app names the FIPS, SP, RFC or ISO document it follows, with the version.
5. **Show the failure modes too.** Nonce reuse, missing authentication, timing leaks and downgrade attacks get their own demos, so the "why" is visible.
6. **Public information only.** Automotive examples are built from public standards (AUTOSAR, ISO/SAE 21434, ISO 14229, IEEE 1609.2, Uptane) and contain nothing from any employer.

## Layout (planned)

```
apps/      Showcase apps, one folder per project
learn/     Educational from-scratch implementations (not for production)
vectors/   Test vectors and the scripts that fetch them
docs/      Write-ups, threat models, design notes
```

## Security

See [SECURITY.md](SECURITY.md) for how to report a problem.

## License

[Apache License 2.0](LICENSE).
