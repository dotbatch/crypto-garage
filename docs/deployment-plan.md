# Deployment plan

How code in this repo reaches the public. Builds on [hosting-plan.md](hosting-plan.md). Written October 2026; nothing here is built yet.

## Domains

| Domain | Today | Use |
|---|---|---|
| `udcio.com` | DNS at Namecheap; points at a server with no website | **Showcase.** Clean, neutral, not tied to the business sites |
| `pixsol.net` | Live "Pix Sol" site on the GoDaddy VPS | Leave alone |
| `pxibit.com` | Live small-business web services site on the GoDaddy VPS | Leave alone. Optionally link to the showcase |

Hostnames (recommended, confirm before setting up DNS):

| Hostname | Served by | Purpose |
|---|---|---|
| `garage.udcio.com` | GitHub Pages | Showcase site and in-browser demos |
| `pq.udcio.com` | GoDaddy VPS | Phase 2: hybrid post-quantum TLS endpoint |

- [ ] Find out what the existing `udcio.com` A record points at, and whether anything still uses it, before changing DNS
- [ ] Decide whether the bare `udcio.com` redirects to `garage.udcio.com` or stays a personal landing page

## Phase 1: showcase site on GitHub Pages

### Pipeline

```
pull request ──> ci.yml: lint, unit tests, known-answer tests, Wycheproof, audit
                          (no deploy)

push to main ──> ci.yml (same checks)
                   │ passes
                   v
                 pages.yml: build site/ + Wasm demos ──> deploy to GitHub Pages
                   │
                   v
                 smoke check: HTTPS 200, CSP present, demo page loads
```

- Deploys only from `main`, and only after CI passes. A failing test blocks the deploy.
- `paths:` filter: the Pages workflow runs only when `site/`, `apps/` or the workflow itself changes.
- No secrets needed. Pages deploys through GitHub's OIDC token (`id-token: write`).

### Workflow outline (`.github/workflows/pages.yml`)

```yaml
on:
  push:
    branches: [main]
    paths: ["site/**", "apps/**", ".github/workflows/pages.yml"]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

concurrency:
  group: pages
  cancel-in-progress: false

jobs:
  build:      # checkout, build Wasm demos, build site, upload-pages-artifact
  deploy:     # needs: build; environment: github-pages; deploy-pages
  smoke:      # needs: deploy; curl the live URL and check status + headers
```

- [ ] Pin every third-party action to a full commit SHA, not a tag (supply-chain hardening; worth a line on the site)
- [ ] Turn on Dependabot for GitHub Actions so the pinned SHAs get update PRs

### One-time setup

1. [ ] Repo Settings → Pages → Source: **GitHub Actions**
2. [ ] Repo Settings → Environments → `github-pages`: allow deploys from `main` only
3. [ ] Account Settings → Pages → **Verify domain** `udcio.com`. GitHub gives a TXT record to add at Namecheap (`_github-pages-challenge-dotbatch.udcio.com`). This stops anyone else claiming the domain on Pages
4. [ ] Namecheap DNS: `CNAME garage → dotbatch.github.io`
5. [ ] Repo Settings → Pages → Custom domain: `garage.udcio.com`, wait for the certificate, then **Enforce HTTPS**
6. [ ] Namecheap DNS: `CAA 0 issue "letsencrypt.org"` on `udcio.com` (GitHub Pages and the VPS both use Let's Encrypt)
7. [ ] Branch protection on `main`: require CI to pass before merge

### Rollback

- Revert the bad commit on `main` and push; the pipeline redeploys the previous state.
- Faster: re-run the last good `pages.yml` run from the Actions tab.

## Phase 2: PQ TLS endpoint on the VPS

### Constraint: the VPS runs live sites

pixsol.net and pxibit.com share this server. cPanel's Apache owns port 443 on the main IP, and the PQ server must not disturb it. Apache can't simply proxy the connection either, because the visitor's TLS handshake (where the post-quantum key exchange happens) has to end at the PQ server itself.

Options, in order of preference:
1. **A second IP on the VPS** (ask GoDaddy). The PQ server takes port 443 on that IP, and `pq.udcio.com` points at it. Cleanest.
2. **Alternate port** (e.g. `https://pq.udcio.com:8443`). No extra cost, but some networks block unusual ports.
3. **A separate small VPS** just for this. Removes all risk to the business sites.

### Steps

- [ ] Take a full WHM backup before any change
- [ ] Inventory: OS version, `openssl version`, root SSH, whether Docker/Podman is allowed
- [ ] Choose option 1, 2 or 3 above
- [ ] Build or install nginx/Caddy against OpenSSL 3.5+ or AWS-LC; run as its own systemd service or container, separate from cPanel
- [ ] Let's Encrypt certificate for `pq.udcio.com`
- [ ] Namecheap DNS: `A pq → <PQ server IP>`
- [ ] Page that reports the negotiated TLS group to the visitor
- [ ] Verify: `openssl s_client -connect pq.udcio.com:443 -groups X25519MLKEM768`, plus current Chrome and Firefox
- [ ] Apply the hardening checklist in [hosting-plan.md](hosting-plan.md)

### Deploys to the VPS

- Start manual: config lives in the repo under `deploy/pq/`, copied to the server by hand.
- Later, a GitHub Action over SSH:
  - [ ] Dedicated non-root `deploy` user, allowed to update only the PQ server's files and restart its service
  - [ ] SSH key stored as a secret in a GitHub environment `vps` that requires manual approval
  - [ ] Never touches cPanel, Apache or the business sites' files
- Rollback: keep the previous config or container tag on the server and switch back.

## Order of work

1. Confirm hostnames, check the existing `udcio.com` record
2. `ci.yml` with tests (needed before anything deploys)
3. `site/` skeleton and `pages.yml`; DNS and HTTPS for `garage.udcio.com`
4. First demo page
5. Phase 2 inventory of the VPS, then pick an option
