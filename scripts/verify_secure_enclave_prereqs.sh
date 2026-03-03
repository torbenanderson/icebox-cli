#!/usr/bin/env bash
set -euo pipefail

BIN_PATH="${1:-target/release/icebox-cli}"

has_match() {
  local pattern="$1"
  local input="$2"
  if command -v rg >/dev/null 2>&1; then
    echo "$input" | rg -qi "$pattern"
  else
    echo "$input" | grep -Eqi "$pattern"
  fi
}

echo "== Icebox Secure Enclave Prereq Check =="
echo "binary: $BIN_PATH"

if [[ ! -f "$BIN_PATH" ]]; then
  echo "FAIL: binary not found at $BIN_PATH"
  echo "Build first: cargo build --release"
  exit 1
fi

fail=0

arch="$(uname -m)"
echo "arch: $arch"
if [[ "$arch" == "arm64" ]]; then
  echo "PASS: Apple Silicon detected"
else
  echo "WARN: non-Apple-Silicon machine detected ($arch)"
fi

if has_match "Apple T2" "$(system_profiler SPiBridgeDataType 2>/dev/null || true)"; then
  echo "PASS: Apple T2 Security Chip detected"
else
  echo "WARN: Apple T2 Security Chip not detected via SPiBridgeDataType"
fi

if codesign --verify --verbose=2 "$BIN_PATH" >/dev/null 2>&1; then
  echo "PASS: code signature verifies"
else
  echo "FAIL: binary is not code-signed (or signature is invalid)"
  fail=1
fi

codesign_info="$(codesign -dvv "$BIN_PATH" 2>&1 || true)"
if has_match "Runtime Version" "$codesign_info"; then
  echo "PASS: hardened runtime present"
else
  echo "WARN: hardened runtime not detected in signature metadata"
fi

entitlements="$(codesign -d --entitlements :- "$BIN_PATH" 2>/dev/null || true)"
if [[ -z "$entitlements" ]]; then
  echo "FAIL: no embedded entitlements found"
  fail=1
else
  echo "PASS: entitlements embedded"
  if has_match "keychain-access-groups" "$entitlements"; then
    echo "PASS: keychain-access-groups entitlement present"
  else
    echo "FAIL: keychain-access-groups entitlement missing"
    fail=1
  fi
fi

echo
if [[ "$fail" -ne 0 ]]; then
  echo "Result: FAIL"
  echo "Hint: sign with entitlements, then run from normal user Terminal session."
  exit 1
fi

echo "Result: PASS"
echo "Prereqs look good for real Secure Enclave verification."
