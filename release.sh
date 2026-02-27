#!/bin/bash
set -e

# Release script for demoji
# Usage: ./release.sh <version>
# Example: ./release.sh 0.2.0

if [ -z "$1" ]; then
    echo "Usage: ./release.sh <version>"
    echo "Example: ./release.sh 0.2.0"
    exit 1
fi

VERSION="$1"
TAG="v$VERSION"

# Validate version format (basic check)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in format X.Y.Z (e.g., 0.2.0)"
    exit 1
fi

# Check for uncommitted changes
if ! git diff --quiet || ! git diff --staged --quiet; then
    echo "Error: You have uncommitted changes. Please commit or stash them first."
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Error: Tag $TAG already exists"
    exit 1
fi

echo "Releasing version $VERSION..."

# Update version in Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update Cargo.lock
cargo update -p demoji

# Commit, tag, and push
git add Cargo.toml Cargo.lock
git commit -m "Release $VERSION"
git tag "$TAG"
git push
git push origin "$TAG"

echo ""
echo "✅ Released $TAG"
echo ""
echo "GitHub Actions will now:"
echo "  • Create a GitHub release with binaries"
echo "  • Update the Homebrew formula"
echo ""
echo "Track progress: gh run list --limit 3"

