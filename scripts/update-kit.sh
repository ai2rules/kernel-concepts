#!/usr/bin/env bash
# Replace kit/ with the conformance kit of a kernel release, and pin that version.
#
#   bash scripts/update-kit.sh v0.9.0
#
# The kernel repository is private, so this needs `gh` signed in with access to
# ai2rules/kernel (the maintainer). Then run the conformance against the npm binary of
# the same version, and commit kit/ with KERNEL_VERSION in one change.
set -euo pipefail
cd "$(dirname "$0")/.."

ver="${1:?usage: update-kit.sh <kernel version, e.g. v0.9.0>}"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

gh release download "$ver" --repo ai2rules/kernel \
  --pattern 'harness-conformance-kit.tar.gz' --pattern 'harness-conformance-kit.tar.gz.sha256' \
  --dir "$tmp"
(cd "$tmp" && sha256sum -c harness-conformance-kit.tar.gz.sha256)
rm -rf kit && mkdir kit
tar -xzf "$tmp/harness-conformance-kit.tar.gz" -C kit
printf '%s\n' "$ver" > KERNEL_VERSION
echo "kit/ now holds the conformance kit of kernel $ver. Next:"
echo "  npm install -g ai2rules-harness@${ver#v} && cargo run -p conformance"
