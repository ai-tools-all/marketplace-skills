#!/usr/bin/env bash
# /doc ticket <index> <NN> <title> — create one ticket file in experiment.
# NN is the ticket's dependency-order number (blockers first). It becomes the
# filename sequence prefix so "Blocked by: NN, NN" references stay stable —
# explicit rather than creation-order auto-numbering.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc ticket <index> <NN> <title>}"
NN_RAW="${2:?Usage: /doc ticket <index> <NN> <title>}"
TITLE="${3:?Usage: /doc ticket <index> <NN> <title>}"

if [[ ! "$NN_RAW" =~ ^[0-9]+$ ]]; then
  echo "ERROR: ticket number must be numeric, got: '$NN_RAW'" >&2
  exit 1
fi
NUM="$(printf '%02d' "$((10#$NN_RAW))")"
SLUG="$(slugify "$TITLE")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
TICKET_DIR="$EXPT_PATH/tickets"
mkdir -p "$TICKET_DIR"

# One ticket per number. The datetime in the filename means an exact-path check
# isn't enough — guard on the NN- prefix so a re-run can't silently duplicate.
for existing in "$TICKET_DIR/${NUM}-"*; do
  [[ -e "$existing" ]] || continue
  echo "ERROR: ticket $NUM already exists: $(basename "$existing")" >&2
  exit 1
done

# Render filename
FILENAME="$(render_name "$(cfg naming.ticket 2>/dev/null || echo '{NN}-{datetime}-{title}.md')" "$NUM" "$SLUG")"

TICKET_PATH="$TICKET_DIR/$FILENAME"

# Frontmatter + the ticket skeleton. YOU fill the sections after this runs.
{
  emit_frontmatter ticket "$TITLE" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME"
  cat <<'EOF'
## What to build

The end-to-end behaviour this ticket makes work, from the user's perspective.

## Acceptance criteria

- [ ]

## Blocked by

None — can start immediately.
EOF
} > "$TICKET_PATH"

# Update meta
update_meta "$EXPT_PATH" "ticket_count" "increment"

echo "Created $EXPT_NAME/tickets/$FILENAME"
echo "  Path: $TICKET_PATH"
