#!/usr/bin/env bash
# Bump version, commit, and tag for release.
# Usage: ./scripts/release.sh 0.2.0

set -euo pipefail

VERSION="${1:?Usage: $0 <version> (e.g. 0.2.0)}"

# Validate semver format
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "❌ Invalid version format: $VERSION (expected X.Y.Z)"
  exit 1
fi

# Update Cargo.toml
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Update Cargo.lock
cargo check --quiet 2>/dev/null

echo "✅ Bumped version to $VERSION"
echo ""
echo "Next steps:"
echo "  git add Cargo.toml Cargo.lock"
echo "  git commit -m \"release: v$VERSION\""
echo "  git tag v$VERSION"
echo "  git push origin main --tags"
