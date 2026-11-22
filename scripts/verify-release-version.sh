#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: verify-release-version.sh <crate-name> <version>" >&2
  exit 2
fi

CRATE_NAME="$1"
EXPECTED_VERSION="$2"

DECLARED_VERSION="$(
  ./scripts/read-crate-version.sh "$CRATE_NAME"
)"

if [ "$DECLARED_VERSION" != "$EXPECTED_VERSION" ]; then
  echo "declared version $DECLARED_VERSION differs from release version $EXPECTED_VERSION" >&2
  exit 1
fi
