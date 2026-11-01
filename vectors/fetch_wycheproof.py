# /// script
# requires-python = ">=3.12"
# ///
"""Fetch Wycheproof test vectors from a pinned commit and verify their SHA-256 checksums.

Run with: uv run vectors/fetch_wycheproof.py

Files already present with the right checksum are left alone, so rerunning is a no-op.
A downloaded file with the wrong checksum stops the run and is never written.
"""

import hashlib
import sys
import urllib.request
from pathlib import Path

# Pinning a commit, not a branch, means upstream changes can't silently alter our vectors.
# To bump: change COMMIT, then replace each checksum with the SHA-256 of the file at that commit.
REPO = "C2SP/wycheproof"
COMMIT = "d17cc89fa5c8f4f4d378b5385ebf75da5d0926c9"
SOURCE_DIR = "testvectors_v1"

# Recorded when COMMIT was pinned. A mismatch means the bytes are not the ones we reviewed.
CHECKSUMS = {
    "aes_gcm_test.json": "985e5ecc172e181eaf49e89508b9470dcf478002eb7e8559c707eb42dc97dfe7",
    "chacha20_poly1305_test.json": "fe61d25f90e1bde4461d00eafe61049e5f29bd999f36b766df9cda90906ad53d",
    "x25519_test.json": "35c3f5231cf25cc640b524d403461deee9e49441d5d915a3a25b2c8ff5adbe7d",
    "hkdf_sha256_test.json": "bb2b462a38b251cb52a2aede706d6d4b62b26864f4e80c95497507ddb07c5f1e",
    "hmac_sha256_test.json": "2d201cfa61d1bf95e6f5d07d96634b4a348b31e8eaa277ad7c8d09677b7a743f",
    "ed25519_test.json": "ea80ffebac085e21cb9d3e903e7fa2b448d96702da5b31f061d0b83b430b74b3",
    "ecdsa_secp256r1_sha256_test.json": (
        "182db4f3e230f6f9fa9f800d2a614dede30284b8e8438bbfe1171905402e9332"
    ),
}

OUT_DIR = Path(__file__).resolve().parent / "wycheproof"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fetch(name: str) -> bytes:
    url = f"https://raw.githubusercontent.com/{REPO}/{COMMIT}/{SOURCE_DIR}/{name}"
    with urllib.request.urlopen(url, timeout=60) as response:
        return response.read()


def main() -> int:
    OUT_DIR.mkdir(exist_ok=True)
    for name, expected in CHECKSUMS.items():
        path = OUT_DIR / name
        # These are hashes of public files, so a plain comparison is fine: there is no secret
        # for a timing difference to leak.
        if path.exists() and sha256(path.read_bytes()) == expected:
            print(f"ok        {name}")
            continue
        status = "replaced" if path.exists() else "fetched"
        data = fetch(name)
        actual = sha256(data)
        if actual != expected:
            print(f"MISMATCH  {name}: expected {expected}, got {actual}", file=sys.stderr)
            return 1
        # Write then rename, so an interrupted run never leaves a partial file in place.
        tmp = path.with_suffix(".tmp")
        tmp.write_bytes(data)
        tmp.replace(path)
        print(f"{status:<9} {name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
