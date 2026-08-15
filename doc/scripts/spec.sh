#!/usr/bin/env bash
# /doc spec <index> <title> — create a spec file in experiment
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc spec <index> <title>}"
TITLE="${2:?Usage: /doc spec <index> <title>}"
SLUG="$(slugify "$TITLE")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
SPEC_DIR="$EXPT_PATH/spec"
mkdir -p "$SPEC_DIR"

# Next spec number
NUM="$(next_number "$SPEC_DIR" 2)"

# Render filename
FILENAME="$(render_name "$(cfg naming.spec 2>/dev/null || echo '{NN}-{datetime}-spec-{title}.md')" "$NUM" "$SLUG")"

SPEC_PATH="$SPEC_DIR/$FILENAME"

# Frontmatter + the spec section skeleton. YOU fill the sections after this runs.
{
  emit_frontmatter spec "$TITLE" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME"
  cat <<'EOF'
## Problem Statement

## Solution

## User Stories

## Implementation Decisions

## Testing Decisions

## Out of Scope

## Further Notes
EOF
} > "$SPEC_PATH"

# Update meta
update_meta "$EXPT_PATH" "spec_count" "increment"

echo "Created $EXPT_NAME/spec/$FILENAME"
echo "  Path: $SPEC_PATH"
