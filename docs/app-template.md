# App README template

Every app in [`apps/`](../apps/) has a README with the sections below, in this order. Copy the
template into `apps/<name>/README.md`, replace the placeholders and delete the comments.

Keep each section short. A reader should learn what the app shows, which standard it follows,
what it protects against and where it stops, without reading the code.

````markdown
# <app name>

<One sentence: what the app does.>

## Goal

<!-- What the demo shows and who it is for. One short paragraph. -->

## Standard

<!--
The documents the app follows, each with its version or date, and the sections that matter.
Name the library that implements each primitive, with its version, and the test vectors it is
checked against. For example:
-->

| Primitive | Standard | Library | Vectors |
|---|---|---|---|
| ML-KEM-768 | FIPS 203 (August 2024) | `ml-kem` 0.x | ACVP `ML-KEM-encapDecap-FIPS203` |

## Threat model

<!--
- Assets: what is being protected (keys, messages, firmware images).
- Attacker: what they can see and do (read the network, modify messages, replay old ones).
- In scope: the attacks the app defends against.
- Out of scope: what it does not try to stop, such as side channels or a compromised host.
-->

## How to run

<!-- The exact commands to build, run and test the app from the repo root. -->

```sh
cargo test -p <crate>
```

## How it breaks

<!--
What happens under misuse or attack, and the test that shows it: tampered ciphertext, wrong
key, bad signature, reused nonce. If the app includes a deliberately broken demo, say which
file it is and why it is unsafe.
-->

## Limitations

<!--
What a reader must not assume: not audited, not constant-time on every platform, simplified
key management, missing features. Say what a production version would need.
-->
````
