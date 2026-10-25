# Claude Code Guidelines for crypto-garage

crypto-garage is a public portfolio of cryptography apps: proven primitives, post-quantum algorithms, and vehicle-security demos. Everything here is read by strangers and judged as the work of a security professional. Correctness and clarity beat cleverness.

## Code Change Philosophy

### Keep Changes Small and Focused
- **One logical change per commit.** Each commit does exactly one thing
- If a task feels too large, break it into subtasks
- Prefer several small commits over one large one
- Run tests after each change, not at the end

**Quality over speed. Small steps compound into big progress.**

### Task Prioritization

When choosing the next task, prioritize in this order:

1. **Architecture and core abstractions:** crate/package layout, the shared crypto interface, the test-vector harness
2. **Integration points:** how apps, `vectors/`, CI and `site/` connect
3. **Unknowns and spikes:** e.g. Wasm builds of PQ libraries, PQ TLS on the VPS, microcontroller toolchains
4. **Standard features:** the demos themselves
5. **Polish and quick wins:** styling, wording, small cleanups

**Fail fast on risky work. Save easy wins for later.**

## Cryptography Rules

These come first. Breaking one is a bug even if the tests pass.

- **No home-grown primitives in `apps/`.** Use vetted libraries: RustCrypto, libsodium, AWS-LC, OpenSSL 3.5+, liboqs. From-scratch implementations go only in `learn/`, with a "not for production" banner at the top of each file
- **Every primitive is tested against known answers.** NIST ACVP/CAVP vectors, RFC vectors and Wycheproof cases from `vectors/`. A new algorithm isn't done until its vectors pass in CI
- **Cite the standard.** Each app's README names the FIPS, SP, RFC or ISO document it follows, with the version
- **Randomness comes from the OS CSPRNG only** (`getrandom`, `OsRng`, `crypto.getRandomValues`). Never `rand()`, `Math.random()` or time-based seeds
- **Compare secrets in constant time.** Never `==` on MACs, tags or hashes of secrets
- **Never log, print or `Debug`-format key material.** Zeroize secrets when the language supports it (e.g. `zeroize` in Rust)
- **Use AEAD, not bare encryption.** Nonces are unique per key; say in the code how uniqueness is guaranteed
- **Deliberately broken code** (failure-mode demos) is labeled at the top of the file and in its README, and never imported by non-demo code
- **When unsure about a cryptographic detail, stop and ask** rather than guess

## Public Repo Rules

- **Public sources only.** Automotive work uses public standards (AUTOSAR, ISO 14229, ISO/SAE 21434, IEEE 1609.2, Uptane). Nothing from any employer: no internal names, part numbers, tools, keys or architecture
- **No secrets in the repo.** No real keys, tokens, passwords, VPS IPs or hostnames beyond the public ones in `docs/`. Test keys are generated at runtime or clearly marked test-only in `vectors/`
- **Don't change live business sites.** pixsol.net and pxibit.com run on the same VPS; nothing in this repo touches them

## Code Quality Standards

### Write Concise Code
After writing any code file, ask: *"Would a senior engineer say this is overcomplicated?"* If yes, **simplify**.

### Avoid Over-Engineering
- Only make changes that are requested or clearly necessary
- Don't add features beyond what was asked
- Don't refactor code that doesn't need it
- A bug fix doesn't need surrounding code cleaned up
- A simple feature doesn't need extra configurability

### Clean Code Practices
- Don't pad files
- Don't leave dead code; if it's unused, delete it
- No backwards-compatibility hacks or `// removed` comments for deleted code
- Demo code is read by learners: prefer clear names and short comments explaining *why*, especially around nonces, key derivation and verification

### Task Decomposition
- Use micro tasks. The smaller the task, the better the code
- Break complex work into discrete, testable units
- Each micro task should be completable in one focused session

## Legacy and Technical Debt

This repo is a public record. Every shortcut is visible, and the patterns here will be copied by people learning cryptography. Copied crypto mistakes travel far.

**Fight entropy. Leave the codebase better than you found it.**

## Project-Specific Rules

### Tech Stack
Not final yet. Current plan:
- Core and embedded: Rust (stable), formatted with `rustfmt`, linted with `clippy -D warnings`
- Browser demos: Rust compiled to WebAssembly
- Teaching notebooks and scripts: Python 3.12+, formatted and linted with `ruff`
- Site: static site generator (Zola is the leading candidate), deployed to GitHub Pages

Update this section once a choice is made.

### Directory Structure
```
apps/      Showcase apps, one folder per project, each with its own README
learn/     Educational from-scratch implementations (not for production)
vectors/   Test vectors and the scripts that fetch them (never hand-edited)
site/      Showcase website source (planned)
deploy/    VPS deployment config (planned)
docs/      Plans, write-ups, threat models
```

### Code Standards
- LF line endings (enforced by `.gitattributes`)
- Line width: 100 characters
- Use each language's standard formatter; don't hand-format
- Run the formatter, linter and tests before committing

### Boundaries: Never Modify
- Files in `vectors/` by hand. Regenerate them with their fetch scripts
- `LICENSE`
- Lock files (`Cargo.lock`, `uv.lock`, etc.) except through the package manager
- GitHub repo settings, DNS, or the VPS without asking first

### Testing
- Write tests for new features
- Known-answer tests for every primitive; Wycheproof cases where they exist
- Include negative tests: tampered ciphertexts, wrong keys and bad signatures must be rejected

## Commit Guidelines

1. One logical change per commit
2. Write descriptive commit messages
3. Format: `type: brief description`
   - `feat:` new feature or demo
   - `fix:` bug fix
   - `refactor:` code restructuring
   - `docs:` documentation and plans
   - `test:` test or test-vector changes
   - `ci:` workflows and deployment pipeline
   - `chore:` maintenance
4. Push to GitHub only when asked. Exception: the daily job ([docs/daily-job.md](docs/daily-job.md)) may push its own `fam-*` branch and open a PR, but never pushes to `main` or merges
