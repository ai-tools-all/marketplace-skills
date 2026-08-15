# Issue tracker: Local Markdown

Specs and tickets for this repo live in the `/doc` skill's experiment structure under `docs/experiments/`. It gives them numbered, frontmatter-stamped files and a per-feature home, which is more systematic than loose `.scratch/` files. Wayfinding maps (see below) still use `.scratch/` because they need a mutable claim/resolve loop that `/doc`'s immutable files don't model.

## Conventions

- One feature per **experiment**: `docs/experiments/NNN-<feature-slug>/`, created with `/doc start "<feature>"`
- The spec lives in `spec/`, created with `/doc spec <idx> "<title>"` — its frontmatter `status:` (`draft`) tracks state
- Implementation tickets live in `tickets/`, one file per ticket, created with `/doc ticket <idx> <NN> "<title>"`, numbered from `01` in dependency order — never a single combined tickets file
- Triage state is the ticket's frontmatter `status:` field — `ready-for-agent` on creation; update it in place as the ticket moves through triage (see `triage-labels.md` for the role strings). This is the one mutable field on an otherwise immutable doc file.
- Blocking edges are the `NN` numbers listed under the ticket's `## Blocked by` heading
- Comments and conversation history append to the bottom of the file under a `## Comments` heading

## When a skill says "publish to the issue tracker"

- A spec → `/doc spec <idx> "<title>"`, then write the spec into the created file.
- A ticket → `/doc ticket <idx> <NN> "<title>"`, then write the ticket into the created file.

Create or resume the feature's experiment first (`/doc start` / resume the active one). `/doc` scaffolds `docs/` on first use — no repo-root setup needed.

## When a skill says "fetch the relevant ticket"

Read the referenced file under `docs/experiments/NNN-<feature-slug>/`. The user will normally pass the path or the ticket number directly; `/doc list` and `/doc status <idx>` locate an experiment if not.

## Wayfinding operations

Used by `/wayfinder`. The **map** is a file with one **child** file per ticket.

- **Map**: `.scratch/<effort>/map.md` — the Notes / Decisions-so-far / Fog body.
- **Child ticket**: `.scratch/<effort>/issues/NN-<slug>.md`, numbered from `01`, with the question in the body. A `Type:` line records the ticket type (`research`/`prototype`/`grilling`/`task`); a `Status:` line records `claimed`/`resolved`.
- **Blocking**: a `Blocked by: NN, NN` line near the top. A ticket is unblocked when every file it lists is `resolved`.
- **Frontier**: scan `.scratch/<effort>/issues/` for files that are open, unblocked, and unclaimed; first by number wins.
- **Claim**: set `Status: claimed` and save before any work.
- **Resolve**: append the answer under an `## Answer` heading, set `Status: resolved`, then append a context pointer (gist + link) to the map's Decisions-so-far in `map.md`.
