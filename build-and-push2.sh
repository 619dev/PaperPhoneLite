#!/usr/bin/env bash
# Build PaperPhoneLite Docker images with Depot and optionally push them.
#
# Usage:
#   ./build-and-push2.sh                         # push version tag and latest
#   TAG=3.0.1 ./build-and-push2.sh              # override the image tag
#   PUSH=0 ./build-and-push2.sh                  # build without pushing
#   REPO=my-dockerhub-user ./build-and-push2.sh  # override image namespace
#
# Required:
#   DEPOT_PROJECT_ID=<your Depot project ID>

set -Eeuo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO="${REPO:-facilisvelox}"
TAG="${TAG:-$(sed -n 's/^version = "\([^"]*\)"/\1/p' "${ROOT_DIR}/server/Cargo.toml" | head -n 1)}"
PUSH="${PUSH:-1}"
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"

if ! command -v depot >/dev/null 2>&1; then
  echo "Error: Depot CLI is not installed or is not available in PATH." >&2
  echo "Install it locally or run this script in CI with depot/setup-action." >&2
  exit 1
fi

if [[ -z "${DEPOT_PROJECT_ID:-}" ]]; then
  echo "Error: DEPOT_PROJECT_ID is required." >&2
  exit 1
fi

if [[ -z "$TAG" ]]; then
  echo "Error: TAG must not be empty." >&2
  exit 1
fi

if [[ "$PUSH" != "0" && "$PUSH" != "1" ]]; then
  echo "Error: PUSH must be 0 or 1." >&2
  exit 1
fi

build_image() {
  local component="$1"
  local context="$2"
  local image="${REPO}/paperphone-lite-${component}"
  local tags=(--tag "${image}:${TAG}")

  if [[ "$TAG" != "latest" ]]; then
    tags+=(--tag "${image}:latest")
  fi

  if [[ "$PUSH" == "1" ]]; then
    echo "Building and pushing ${image}:${TAG} for ${PLATFORMS} with Depot..."
    depot build \
      --project "$DEPOT_PROJECT_ID" \
      --platform "$PLATFORMS" \
      "${tags[@]}" \
      --push \
      "$context"
  else
    echo "Building ${image}:${TAG} for ${PLATFORMS} with Depot (no push)..."
    depot build \
      --project "$DEPOT_PROJECT_ID" \
      --platform "$PLATFORMS" \
      "${tags[@]}" \
      "$context"
  fi
}

build_image server "${ROOT_DIR}/server"
build_image tor "${ROOT_DIR}/deploy/tor"

echo "Done. Published tag: ${TAG}"
if [[ "$TAG" != "latest" ]]; then
  echo "The same images were also tagged as latest."
fi
