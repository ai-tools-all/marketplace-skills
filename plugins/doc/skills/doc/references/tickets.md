# Tickets → one checklist — folded in from `to-tickets`

Break a plan, spec, or conversation into **tracer-bullet vertical slices**, then write
them as ONE ordered checklist into the experiment's `tasks.md` (created by `/doc tasks <idx>`).
Locally there is no one-file-per-ticket — it's a single mutable checklist you tick in place
as `/implement` lands each slice.

## Process

### 1. Gather context

Work from whatever is already in the conversation context. If the user passes a reference
(a spec path, an issue number or URL), fetch it and read its full body and comments.

### 2. Explore the codebase (optional)

If you have not already, explore to understand the current state of the code. Titles should
use the project's domain glossary vocabulary and respect ADRs in the area you're touching.

Look for opportunities to prefactor to make the implementation easier. "Make the change easy,
then make the easy change."

### 3. Draft vertical slices

<vertical-slice-rules>

- Each slice cuts a narrow but COMPLETE path through every layer (schema, API, UI, tests) —
  vertical, NOT a horizontal slice of one layer
- A completed slice is demoable or verifiable on its own
- Each slice is sized to fit in a single fresh context window
- Any prefactoring should be done first

</vertical-slice-rules>

Give each slice its **blocking edges** — the other slices that must complete before it can
start. A slice with no blockers can start immediately.

**Wide refactors are the exception to vertical slicing.** A wide refactor is one mechanical
change — rename a column, retype a shared symbol — whose blast radius fans across the whole
codebase, so a single edit breaks thousands of call sites at once and no vertical slice can
land green. Sequence it as **expand–contract**: first expand (add the new form beside the old
so nothing breaks), then migrate call sites in batches sized by blast radius (per package, per
directory — each batch its own checklist item, blocked by the expand, CI green batch to batch
because the old form still exists), finally contract (delete the old form once no caller
remains, blocked by every migrate batch). When even the batches can't stay green alone, keep
the sequence but let them share an integration branch that all block a final
integrate-and-verify item — green is promised only there.

### 4. Quiz the user

Present the proposed breakdown as a numbered list. For each slice show:

- **Title**: short descriptive name
- **Blocked by**: which other slices (if any) must complete first
- **What it delivers**: the end-to-end behaviour this slice makes work

Ask: Does the granularity feel right (too coarse / too fine)? Are the blocking edges correct?
Should any be merged or split? Iterate until the user approves.

### 5. Write the checklist

Run `/doc tasks <idx>` once, then write the approved slices into `tasks.md` as an ordered
checklist, **blockers first**. Each item is one tracer-bullet slice; note `(blocked by N)`
inline where an item depends on an earlier one. Tick boxes in place as each slice lands.

```markdown
## Tasks

- [ ] 1. Schema + migration for X
- [ ] 2. API endpoint reading X (blocked by 1)
- [ ] 3. UI wired to the endpoint (blocked by 2)
- [ ] 4. Tests across the slice (blocked by 3)
```

Work the **frontier**: any item whose blockers are all ticked. For a purely linear chain that
means top to bottom.

When a slice needs more than a line, expand that item with the shape below (kept compact —
it's an entry in a list, not a file):

<slice-detail>

**What to build** — the end-to-end behaviour this slice makes work, from the user's
perspective; not a layer-by-layer implementation list.

**Acceptance criteria** — a couple of checkable outcomes.

**Blocked by** — the item numbers that gate this one, or "None — can start immediately".

</slice-detail>

Avoid specific file paths or code snippets — they go stale fast. Exception: if a prototype
produced a snippet that encodes a decision more precisely than prose can (state machine,
reducer, schema, type shape), inline it and note briefly that it came from a prototype.
