# /// script
# requires-python = ">=3.12"
# ///
"""Fetch test vectors from pinned upstream commits and verify their SHA-256 checksums.

Run with: uv run vectors/fetch_vectors.py

Files already present with the right checksum are left alone, so rerunning is a no-op.
A downloaded file with the wrong checksum stops the run and is never written.
"""

import hashlib
import sys
import urllib.request
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Source:
    repo: str
    # Pinning a commit, not a branch, means upstream changes can't silently alter our vectors.
    # To bump: change the commit, then replace each checksum with the SHA-256 of the file there.
    commit: str
    # Directory in the upstream repo that file names are relative to.
    upstream_dir: str
    # Directory under vectors/ the files are written to, keeping their upstream relative paths.
    local_dir: str
    # Recorded when the commit was pinned. A mismatch means the bytes are not the ones we reviewed.
    checksums: dict[str, str]


SOURCES = [
    Source(
        repo="C2SP/wycheproof",
        commit="d17cc89fa5c8f4f4d378b5385ebf75da5d0926c9",
        upstream_dir="testvectors_v1",
        local_dir="wycheproof",
        checksums={
            "aes_gcm_test.json": "985e5ecc172e181eaf49e89508b9470dcf478002eb7e8559c707eb42dc97dfe7",
            "chacha20_poly1305_test.json": (
                "fe61d25f90e1bde4461d00eafe61049e5f29bd999f36b766df9cda90906ad53d"
            ),
            "x25519_test.json": "35c3f5231cf25cc640b524d403461deee9e49441d5d915a3a25b2c8ff5adbe7d",
            "hkdf_sha256_test.json": (
                "bb2b462a38b251cb52a2aede706d6d4b62b26864f4e80c95497507ddb07c5f1e"
            ),
            "hmac_sha256_test.json": (
                "2d201cfa61d1bf95e6f5d07d96634b4a348b31e8eaa277ad7c8d09677b7a743f"
            ),
            "ed25519_test.json": "ea80ffebac085e21cb9d3e903e7fa2b448d96702da5b31f061d0b83b430b74b3",
            "ecdsa_secp256r1_sha256_test.json": (
                "182db4f3e230f6f9fa9f800d2a614dede30284b8e8438bbfe1171905402e9332"
            ),
        },
    ),
    # internalProjection.json holds each case's inputs and expected outputs in one file, unlike
    # the prompt.json / expectedResults.json pair that a validation client sees.
    Source(
        repo="usnistgov/ACVP-Server",
        commit="975de31eb83d87039ec88934fdc47d8c312b892d",
        upstream_dir="gen-val/json-files",
        local_dir="acvp",
        checksums={
            "ACVP-AES-GCM-1.0/internalProjection.json": (
                "0703118c1f751f32cdb8d32b8e54d5f80eb9bbe7bb99b83a6b0221cb40b34f0e"
            ),
            "ML-KEM-keyGen-FIPS203/internalProjection.json": (
                "d7a62a2c3476957f56dd8d24f9004ea6776ccfe995ffe71a65bb9506dc9c7b1b"
            ),
            "ML-KEM-encapDecap-FIPS203/internalProjection.json": (
                "a556952ce869bb89c3a3196a701dad89647c193a34c86eafb61a9d710d5b810f"
            ),
            "ML-DSA-keyGen-FIPS204/internalProjection.json": (
                "e67ee6540d40e11506c3c4e3b1f79fc1cefcd49820db99fc61f87cc8ba463baf"
            ),
            "ML-DSA-sigGen-FIPS204/internalProjection.json": (
                "72dcaf5f69853ca267ccd16af9cb40949786aca0fcfbf05d1ebeba132b93af22"
            ),
            "ML-DSA-sigVer-FIPS204/internalProjection.json": (
                "47cdd6314c7f746d02421ffcba89d4dbc7bb875ac49e07a029fdfc26fba55437"
            ),
            "SHA2-256-1.0/internalProjection.json": (
                "a7e0bf4f5c661f1787b343bf3ae1a809b1e62f8fb2c8a627791510d03a570a6c"
            ),
        },
    ),
]

VECTORS_DIR = Path(__file__).resolve().parent


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fetch(source: Source, name: str) -> bytes:
    url = (
        f"https://raw.githubusercontent.com/{source.repo}/{source.commit}/"
        f"{source.upstream_dir}/{name}"
    )
    with urllib.request.urlopen(url, timeout=60) as response:
        return response.read()


def main() -> int:
    for source in SOURCES:
        for name, expected in source.checksums.items():
            label = f"{source.local_dir}/{name}"
            path = VECTORS_DIR / label
            # These are hashes of public files, so a plain comparison is fine: there is no secret
            # for a timing difference to leak.
            if path.exists() and sha256(path.read_bytes()) == expected:
                print(f"ok        {label}")
                continue
            status = "replaced" if path.exists() else "fetched"
            data = fetch(source, name)
            actual = sha256(data)
            if actual != expected:
                print(f"MISMATCH  {label}: expected {expected}, got {actual}", file=sys.stderr)
                return 1
            path.parent.mkdir(parents=True, exist_ok=True)
            # Write then rename, so an interrupted run never leaves a partial file in place.
            tmp = path.with_suffix(".tmp")
            tmp.write_bytes(data)
            tmp.replace(path)
            print(f"{status:<9} {label}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
