#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: publish-rust-crate.sh <crate-name> <version>" >&2
  exit 2
fi

CRATE_NAME="$1"
RELEASE_VERSION="$2"

DECLARED_VERSION="$(
  ./scripts/read-crate-version.sh "$CRATE_NAME"
)"

if [ "$DECLARED_VERSION" != "$RELEASE_VERSION" ]; then
  echo "declared version $DECLARED_VERSION differs from release version $RELEASE_VERSION" >&2
  exit 3
fi

STATUS="$(
  curl \
    --silent \
    --show-error \
    --output /dev/null \
    --write-out '%{http_code}' \
    --header 'User-Agent: standardizer-common-release-automation (https://github.com/BezotCorp/standardizer-common)' \
    "https://crates.io/api/v1/crates/$CRATE_NAME/$RELEASE_VERSION"
)"

case "$STATUS" in
  200)
    echo "$CRATE_NAME $RELEASE_VERSION is already published on crates.io."
    exit 0
    ;;
  404)
    echo "Publishing $CRATE_NAME $RELEASE_VERSION to crates.io..."
    cargo publish --locked --allow-dirty
    ;;
  *)
    echo "crates.io returned HTTP $STATUS for $CRATE_NAME $RELEASE_VERSION" >&2
    exit 3
    ;;
esac
