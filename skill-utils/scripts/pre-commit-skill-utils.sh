#!/usr/bin/env bash
set -euo pipefail

# Pre-commit hook helper for Claude plugin changes.
# Runs validation, version bump, and marketplace update for staged plugin manifests.

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." && pwd)"

SKILL_UTILS_BIN="${HOME}/bin/skill-utils"
MARKETPLACE_PATH="${REPO_ROOT}/.claude-plugin/marketplace.json"

run_skill_utils() {
    if [ -x "${SKILL_UTILS_BIN}" ]; then
        "${SKILL_UTILS_BIN}" "$@"
    else
        (cd "${REPO_ROOT}" && cargo run -p skill-utils --release -- "$@")
    fi
}

mapfile -t manifests < <(cd "${REPO_ROOT}" && git diff --cached --name-only -- 'plugins/*/.claude-plugin/plugin.json' | sort -u)

if [ "${#manifests[@]}" -eq 0 ]; then
    exit 0
fi

for manifest_rel in "${manifests[@]}"; do
    manifest_abs="${REPO_ROOT}/${manifest_rel}"
    if [ ! -f "${manifest_abs}" ]; then
        echo "Skipping missing manifest: ${manifest_rel}" >&2
        continue
    fi

    echo "Validating ${manifest_rel}..."
    run_skill_utils validate --manifest "${manifest_abs}"

    echo "Bumping version for ${manifest_rel}..."
    run_skill_utils bump-version --manifest "${manifest_abs}" --level patch

    echo "Updating marketplace for ${manifest_rel}..."
    run_skill_utils add-to-marketplace --manifest "${manifest_abs}" --root "${REPO_ROOT}"

    (cd "${REPO_ROOT}" && git add "${manifest_rel}" "${MARKETPLACE_PATH}")
done

echo "pre-commit skill-utils checks completed."

