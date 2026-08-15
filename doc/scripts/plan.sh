#!/usr/bin/env bash
# /doc plan <index> <title> — create a plan file in experiment
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc plan <index> <title>}"
TITLE="${2:?Usage: /doc plan <index> <title>}"
SLUG="$(slugify "$TITLE")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
PLAN_DIR="$EXPT_PATH/plans"
mkdir -p "$PLAN_DIR"

# Next plan number
NUM="$(next_number "$PLAN_DIR" 2)"

# Render filename
FILENAME="$(render_name "$(cfg naming.plan 2>/dev/null || echo '{NN}-{datetime}-{title}.md')" "$NUM" "$SLUG")"

PLAN_PATH="$PLAN_DIR/$FILENAME"

emit_frontmatter plan "$TITLE" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME" > "$PLAN_PATH"

# Update meta
update_meta "$EXPT_PATH" "plan_count" "increment"

echo "Created $EXPT_NAME/plans/$FILENAME"
echo "  Path: $PLAN_PATH"
