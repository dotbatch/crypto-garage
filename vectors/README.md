# vectors

Known-answer test vectors (NIST ACVP/CAVP, RFC and Wycheproof) and the scripts that fetch them.

Vector files are never edited by hand. To update them, rerun their fetch script.

All vector sets are fetched from pinned upstream commits and checked against recorded SHA-256
checksums by one script:

```sh
uv run vectors/fetch_vectors.py
```

The script is safe to rerun: files that already match are left alone. If a download doesn't match
its checksum, the script exits with an error and writes nothing for that file. The pinned commits,
the file lists and the checksums are at the top of
[`fetch_vectors.py`](fetch_vectors.py).

## Using vectors in Rust tests

This directory is also the `vectors` crate, which loads the files into typed structs. Add it as a
dev-dependency:

```toml
[dev-dependencies]
vectors = { path = "../../vectors" }
```

Then a test runs every case of a file in one loop:

```rust
use vectors::wycheproof::AeadFile;

for (group, test) in AeadFile::load("wycheproof/aes_gcm_test.json").cases() {
    // group: key, IV and tag sizes; test: key, iv, aad, msg, ct, tag and the expected result
}
```

Paths are relative to `vectors/`. `vectors::wycheproof` has one file type per Wycheproof schema,
and `vectors::acvp` one per ACVP set listed below.

## Wycheproof

[`wycheproof/`](wycheproof/) holds JSON test vectors from
[C2SP/wycheproof](https://github.com/C2SP/wycheproof) (Apache-2.0).

## NIST ACVP

[`acvp/`](acvp/) holds known-answer vectors from the NIST
[ACVP-Server](https://github.com/usnistgov/ACVP-Server) `gen-val/json-files/` sample sets, with
the upstream directory names kept. Each `internalProjection.json` has the inputs and expected
outputs of every case:

| Directory | Algorithm | Standard |
|---|---|---|
| `ACVP-AES-GCM-1.0` | AES-GCM encrypt and decrypt | NIST SP 800-38D (2007) |
| `ML-KEM-keyGen-FIPS203` | ML-KEM-512/768/1024 key generation | FIPS 203 (August 2024) |
| `ML-KEM-encapDecap-FIPS203` | ML-KEM encapsulation and decapsulation | FIPS 203 (August 2024) |
| `ML-DSA-keyGen-FIPS204` | ML-DSA-44/65/87 key generation | FIPS 204 (August 2024) |
| `ML-DSA-sigGen-FIPS204` | ML-DSA signing | FIPS 204 (August 2024) |
| `ML-DSA-sigVer-FIPS204` | ML-DSA verification, including invalid signatures | FIPS 204 (August 2024) |

The keys in these files are public test data from NIST, not secrets. ACVP-Server is NIST-developed
software provided as a public service; its README asks that NIST be acknowledged as the source.
