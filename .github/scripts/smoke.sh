#!/usr/bin/env bash
# Post-deploy smoke check: the live site answers over HTTPS, every checked page carries the strict
# CSP, and the Wasm demo's module is served so browsers can load it.
#
# Usage: bash .github/scripts/smoke.sh [base-url]
# The base URL defaults to base_url in site/config.toml.
set -uo pipefail

base=${1:-$(sed -n 's/^base_url = "\(.*\)"$/\1/p' site/config.toml)}
base=${base%/}

# Pages' CDN caches responses for up to 10 minutes. A query string unique to this run skips the
# cache, so the check sees what was just deployed.
bust="smoke=${GITHUB_SHA:-local-$$}"

fail() {
  echo "FAIL: $*" >&2
  return 1
}

# Prints the page body, or fails unless the status is 200.
fetch() {
  local body status
  body=$(curl -sS --max-time 20 -w '\n%{http_code}' "$base$1?$bust") || return 1
  status=${body##*$'\n'}
  [ "$status" = 200 ] || fail "$1 returned HTTP $status" || return 1
  printf '%s' "${body%$'\n'*}"
}

# Prints the content of the page's CSP meta tag (the tag spans two lines), or nothing if it has
# none, so the caller can say what is missing.
csp_of() {
  local tag='http-equiv="Content-Security-Policy"[[:space:]]*content="[^"]*"'
  tr '\n' ' ' | { grep -o "$tag" || :; } | sed 's/.*content="//; s/"$//'
}

check_page() {
  local path=$1 want_scripts=$2 csp
  csp=$(fetch "$path" | csp_of) || return 1
  [ -n "$csp" ] || fail "$path has no CSP meta tag" || return 1
  [[ $csp == "default-src 'none';"* ]] || fail "$path CSP doesn't start with default-src 'none'" ||
    return 1
  [[ $csp != *"'unsafe-"* ]] || fail "$path CSP allows an unsafe- source: $csp" || return 1
  if [ "$want_scripts" = yes ]; then
    [[ $csp == *"script-src 'self' 'wasm-unsafe-eval'"* ]] ||
      fail "$path CSP doesn't allow its Wasm demo: $csp" || return 1
  else
    [[ $csp != *script-src* ]] || fail "$path CSP allows scripts: $csp" || return 1
  fi
  echo "ok: $path CSP: $csp"
}

# instantiateStreaming needs the module served as application/wasm.
check_wasm() {
  local path=$1 type
  type=$(curl -sS --max-time 20 -o /dev/null -w '%{http_code} %{content_type}' \
    "$base$path?$bust") || return 1
  [ "$type" = "200 application/wasm" ] || fail "$path returned '$type'" || return 1
  echo "ok: $path is served as application/wasm"
}

# Enforce HTTPS: plain HTTP must redirect to the HTTPS site.
check_https_redirect() {
  local http=${base/#https:/http:} result
  result=$(curl -sS --max-time 20 -o /dev/null -w '%{http_code} %{redirect_url}' "$http/") ||
    return 1
  [ "$result" = "301 $base/" ] || fail "http:// got '$result', want a 301 to $base/" || return 1
  echo "ok: http:// redirects to $base/"
}

run_checks() {
  local status=0
  check_page / no || status=1
  check_page /about/ no || status=1
  check_page /projects/hello-wasm/ yes || status=1
  check_wasm /demos/hello-wasm/pkg/hello_wasm_bg.wasm || status=1
  if [[ $base == https://* ]]; then
    check_https_redirect || status=1
  fi
  return $status
}

# A deploy can take a minute or two to reach every CDN edge, so retry before failing the run.
for attempt in 1 2 3 4 5; do
  echo "Checking $base (attempt $attempt)"
  run_checks && exit 0
  [ "$attempt" = 5 ] || sleep 30
done
exit 1
