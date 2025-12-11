#!/usr/bin/env bash
set -euo pipefail

# Build the skill-utils binary in release mode and copy it to ~/bin.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
BIN_NAME="skill-utils"
BUILD_TARGET="${REPO_ROOT}/target/release/${BIN_NAME}"
DEST_DIR="${HOME}/bin"

echo "Building ${BIN_NAME} in release mode..."
cargo build --manifest-path "${REPO_ROOT}/Cargo.toml" -p "${BIN_NAME}" --release

echo "Ensuring destination directory exists at ${DEST_DIR}..."
mkdir -p "${DEST_DIR}"

echo "Copying ${BUILD_TARGET} to ${DEST_DIR}/${BIN_NAME}..."
install -m 755 "${BUILD_TARGET}" "${DEST_DIR}/${BIN_NAME}"

echo "Done."

