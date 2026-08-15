#!/usr/bin/env bash
# /doc research <index> <topic> — create research files in experiment
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc research <index> <topic>}"
TOPIC="${2:?Usage: /doc research <index> <topic>}"
SLUG="$(slugify "$TOPIC")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
RESEARCH_DIR="$EXPT_PATH/research"
mkdir -p "$RESEARCH_DIR"

# Next research number
NUM="$(next_number "$RESEARCH_DIR" 2)"

# Render filenames — pin one datetime so the pair stays matched
DT="$(datetimestamp)"
PROMPT_NAME="$(render_name "$(cfg naming.research_prompt 2>/dev/null || echo '{NN}-{datetime}-prompt-{topic}.md')" "$NUM" "$SLUG" "$DT")"
RESPONSE_NAME="$(render_name "$(cfg naming.research_response 2>/dev/null || echo '{NN}-{datetime}-res-{topic}.md')" "$NUM" "$SLUG" "$DT")"

emit_frontmatter research "$TOPIC" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME" "role=prompt" \
  > "$RESEARCH_DIR/$PROMPT_NAME"

emit_frontmatter research "$TOPIC" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME" "role=response" \
  > "$RESEARCH_DIR/$RESPONSE_NAME"

# Update meta
update_meta "$EXPT_PATH" "research_count" "increment"

echo "Created $EXPT_NAME/research/$PROMPT_NAME"
echo "Created $EXPT_NAME/research/$RESPONSE_NAME"
