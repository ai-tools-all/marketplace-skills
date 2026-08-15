#!/usr/bin/env bash
# /doc tasks <index> <NN> — create (once) the task tracker for ONE spec/plan.
# NN is the plan's sequence number: the tracker is keyed to plans/NN-* so the
# spec<->tasks link is explicit, and each spec carries its own tracker.
#
# Unlike plans/findings/etc, this file is MUTABLE: you tick its boxes in place
# as you implement. Idempotent — re-running never clobbers an existing tracker,
# it just prints the path.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/_common.sh"

INDEX="${1:?Usage: /doc tasks <index> <NN>}"
NN_RAW="${2:?Usage: /doc tasks <index> <NN>  (NN = the plan/spec number)}"

if [[ ! "$NN_RAW" =~ ^[0-9]+$ ]]; then
  echo "ERROR: plan number must be numeric, got: '$NN_RAW'" >&2
  exit 1
fi
NN="$(printf '%02d' "$((10#$NN_RAW))")"

EXPT_PATH="$(resolve_experiment "$INDEX")"
EXPT_NAME="$(basename "$EXPT_PATH")"
PLAN_DIR="$EXPT_PATH/plans"
TASKS_DIR="$EXPT_PATH/tasks"
mkdir -p "$TASKS_DIR"

# Find the plan this tracker belongs to, so the title matches and we can warn if
# the spec doesn't exist yet.
PLAN_FILE=""
for p in "$PLAN_DIR/${NN}-"*; do
  [[ -e "$p" ]] && PLAN_FILE="$(basename "$p")" && break
done
if [[ -z "$PLAN_FILE" ]]; then
  echo "WARNING: no plan ${NN}-* in $EXPT_NAME/plans — write the spec first (/doc plan)." >&2
fi

# One tracker per plan NN. The datetime in the filename means an exact-path check
# isn't enough — guard on the NN- prefix so a re-run can't silently duplicate.
for existing in "$TASKS_DIR/${NN}-"*; do
  [[ -e "$existing" ]] || continue
  echo "Task tracker for spec $NN already exists — tick boxes in place, don't recreate."
  echo "  Path: $existing"
  exit 0
done

# Title carries the plan slug when we found one, so the tracker reads as "the
# tasks for <that spec>".
if [[ -n "$PLAN_FILE" ]]; then
  # strip NN- and datetime prefix and .md suffix -> the plan's slug
  SLUG="$(echo "${PLAN_FILE%.md}" | sed -E 's/^[0-9]+-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]{2}-[0-9]{2}-[0-9]{2}-//')"
else
  SLUG="spec-$NN"
fi
TITLE="tasks for spec $NN"

FILENAME="$(render_name "$(cfg naming.tasks 2>/dev/null || echo '{NN}-{datetime}-tasks-{title}.md')" "$NN" "$SLUG")"
TASKS_PATH="$TASKS_DIR/$FILENAME"

# Frontmatter carries `spec: NN` (the link) + status. Body seeds the three
# sections this tracker exists to answer: finished?, deferred?, parallelisable?
{
  emit_frontmatter tasks "$TITLE" "$(expt_topic "$EXPT_NAME")" "experiments/$EXPT_NAME/plans/${PLAN_FILE:-$NN}" "spec=$NN"
  cat <<'EOF'
## Tasks

Ordered checklist for THIS spec — blockers first. Each item is one tracer-bullet
vertical slice (schema -> API -> UI -> tests). Note "(blocked by N)" where an
item depends on an earlier one. Any item whose blockers are all ticked is on the
frontier — those run in parallel. Tick boxes in place as each slice lands.
See the doc skill's references/tickets.md for how to slice.

- [ ] 1.

## Deferred (not this spec)

Slices consciously pushed out of this spec — recorded so they're not lost. Each
can become the next spec.

-
EOF
} > "$TASKS_PATH"

update_meta "$EXPT_PATH" "has_tasks" "true"

echo "Created $EXPT_NAME/tasks/$FILENAME  (tracker for spec $NN)"
echo "  Path: $TASKS_PATH"
