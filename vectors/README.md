# vectors

Known-answer test vectors (NIST ACVP/CAVP, RFC and Wycheproof) and the scripts that fetch them.

Vector files are never edited by hand. To update them, rerun their fetch script.

## Wycheproof

[`wycheproof/`](wycheproof/) holds JSON test vectors from
[C2SP/wycheproof](https://github.com/C2SP/wycheproof) (Apache-2.0), fetched from a pinned commit
and checked against recorded SHA-256 checksums:

```sh
uv run vectors/fetch_vectors.py
```

The script is safe to rerun: files that already match are left alone. If a download doesn't match
its checksum, the script exits with an error and writes nothing for that file. The pinned commit,
the file list and the checksums are at the top of
[`fetch_vectors.py`](fetch_vectors.py).
