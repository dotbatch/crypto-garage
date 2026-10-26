# Hosting plan

How crypto-garage gets shown to the world. Decided October 2026; nothing here is built yet.

## Decision

- **The repo stays the source of truth.** Code, docs and CI live in `github.com/dotbatch/crypto-garage`.
- **A static showcase site on GitHub Pages**, under a custom subdomain, built and deployed by GitHub Actions on every push to `main`.
- **The GoDaddy VPS (cPanel/WHM) is used only for things a static site cannot do**, starting with a live post-quantum TLS endpoint.

```
github.com/dotbatch/crypto-garage     <- code, docs, CI
        | GitHub Actions builds site/ on push to main
        v
crypto.<domain>  (GitHub Pages)       <- write-ups and in-browser demos
pq.<domain>      (VPS)                <- later: hybrid PQ TLS server
```

## Why

- **Demos run in the browser.** Rust compiled to WebAssembly, so keys never leave the visitor's machine. Say this on the site; it is part of the security story.
- **Small attack surface.** No PHP, cPanel or server to patch for the main site. A defaced site would be bad for a security portfolio.
- **Free HTTPS, custom domain, automatic deploys.** The public CI shows the site is built from tested code.

## Domain

**Decided:** `udcio.com`, with `garage.udcio.com` for the site and `pq.udcio.com` for the VPS endpoint. pixsol.net and pxibit.com are live business sites and stay as they are. See [deployment-plan.md](deployment-plan.md) for details and DNS steps.

- Avoid employer names (e.g. GM) and anything that reads as cryptocurrency.

## Site structure

- Homepage: short intro, then a grid of projects.
- One page per project:
  1. What it is and the standard it follows (with version)
  2. Live in-browser demo
  3. How it works, and how it breaks (failure-mode demo)
  4. Link to the code in the repo

## Phase 1: static site on GitHub Pages

- [x] Pick a static site generator (Zola: fits a Rust-heavy repo)
- [ ] Create `site/` in the repo
- [ ] Add a GitHub Actions workflow that builds `site/` (and any Wasm demos) and deploys to Pages; use a `paths:` filter so it only runs when `site/` or `apps/` change
- [ ] Settings → Pages: source = GitHub Actions
- [ ] Add the custom domain in Settings → Pages, then a `CNAME` DNS record at Namecheap (udcio.com's DNS host) pointing the subdomain to `dotbatch.github.io`
- [ ] Verify the domain in GitHub account settings (stops anyone else claiming it on Pages)
- [ ] Turn on "Enforce HTTPS"
- [ ] Add a Content Security Policy via `<meta>` tag (Pages cannot set custom headers); allow `wasm-unsafe-eval` only where Wasm demos need it
- [ ] First demo page: AEAD playground or ML-KEM explorer

## Phase 2: post-quantum TLS endpoint on the VPS

Goal: `pq.<domain>` offers `X25519MLKEM768` so visitors with current Chrome or Firefox negotiate a hybrid post-quantum key exchange, and the page shows them it happened.

- [ ] Check what the VPS runs: OS version, `openssl version`, EasyApache/Apache version, whether root SSH is available
- [ ] cPanel's bundled Apache/OpenSSL likely cannot offer hybrid PQ groups. Plan for a separate nginx or Caddy built against OpenSSL 3.5+ or AWS-LC, either:
  - on its own IP or port alongside cPanel, or
  - in a container (Docker/Podman), if the VPS allows it
- [ ] Certificate from Let's Encrypt for `pq.<domain>`
- [ ] Page that reports the negotiated TLS group back to the visitor
- [ ] Confirm with `openssl s_client -groups X25519MLKEM768` and a browser

## Hardening checklist (anything on the VPS)

- [ ] Keep WHM, cPanel and the OS updated (enable automatic updates)
- [ ] SSH: keys only, no root password login, non-default port optional
- [ ] Firewall: only the ports actually needed (CSF in WHM, or firewalld)
- [ ] TLS 1.3 preferred, TLS 1.2 minimum; Mozilla "modern" config where possible
- [ ] Headers: HSTS (with preload once stable), CSP, `X-Content-Type-Options`, `Referrer-Policy`, `Permissions-Policy`
- [ ] Target A+ on securityheaders.com and the Mozilla HTTP Observatory, and A+ on SSL Labs
- [ ] DNS: CAA record limiting certificate issuers; DNSSEC if GoDaddy supports it for the domain
- [ ] Write up the hardening as its own page on the site

## Alternative considered

Hosting the whole site on the VPS, deployed from GitHub Actions over SSH/rsync. Workable, but it adds patching and hardening for the main site. Revisit only if Pages becomes limiting (e.g. needing real response headers).
