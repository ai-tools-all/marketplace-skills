#!/usr/bin/env bash
# /doc finding <index> <title> — create a finding file in experiment
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc finding <index> <title>}"
TITLE="${2:?Usage: /doc finding <index> <title>}"
SLUG="$(slugify "$TITLE")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
FINDING_DIR="$EXPT_PATH/findings"
mkdir -p "$FINDING_DIR"

# Next finding number
NUM="$(next_number "$FINDING_DIR" 2)"

# Render filename
FILENAME="$(render_name "$(cfg naming.finding 2>/dev/null || echo '{NN}-{datetime}-{title}.md')" "$NUM" "$SLUG")"

FINDING_PATH="$FINDING_DIR/$FILENAME"

emit_frontmatter finding "$TITLE" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME" > "$FINDING_PATH"

# Update meta
update_meta "$EXPT_PATH" "finding_count" "increment"

echo "Created $EXPT_NAME/findings/$FILENAME"
echo "  Path: $FINDING_PATH"
