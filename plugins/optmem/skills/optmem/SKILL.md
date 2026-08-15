---
name: optmem
description: Permanent, append-only memory for AI agents — a project-local memory that survives sessions, compaction, and model/vendor changes. Run `wake` at startup, `note` to record, `recall`/`zoom` to retrieve. One Python file, no dependencies.
argument-hint: "[wake|note|nap|recall|zoom|forget|config|init] [args]"
allowed-tools: Bash(python3 ${AGENTS_SKILLS_DIR}/memo:*), Bash(${AGENTS_SKILLS_DIR}/memo:*), Read
metadata:
  author: "Victor Taelin (OptMem, github.com/VictorTaelin/OptMem); adapted as a skill"
  version: "1.0"
---

# OptMem — Permanent Agent Memory

Project-local, append-only memory. The tool is a single dependency-free Python 3
file (`${AGENTS_SKILLS_DIR}/memo`). Memories live in `$PWD/.optmem/memory` — each
repo carries its own memory, the same way the `/doc` skill scopes docs to the
current project. Set `$MEMORY_DIR` to relocate it (a synced folder, a shared git
repo, or the classic machine-wide `~/.optmem/memory`).

OptMem outlives every session, compaction, model and vendor change. Without it
you do not know who you are, or what was decided and tried.

## Commands

| Command | What to run |
|---------|-------------|
| wake | `python3 ${AGENTS_SKILLS_DIR}/memo wake` |
| note \<text\> | `python3 ${AGENTS_SKILLS_DIR}/memo note "<1 line, max 280 bytes>"` |
| nap | `python3 ${AGENTS_SKILLS_DIR}/memo nap` |
| recall \<regex\> | `python3 ${AGENTS_SKILLS_DIR}/memo recall <regex>` |
| zoom \<lo-hi\> | `python3 ${AGENTS_SKILLS_DIR}/memo zoom <lo>-<hi>` |
| forget \<lo-hi\> | `python3 ${AGENTS_SKILLS_DIR}/memo forget <lo>-<hi>` |
| config | `python3 ${AGENTS_SKILLS_DIR}/memo config [NAME=N]` |
| init | `python3 ${AGENTS_SKILLS_DIR}/memo init` |

**`init` creates `$PWD/.optmem/memory` and prints the setup block.** Run it once
per project. Every other command refuses to run until the memory exists, so a
mistyped `$MEMORY_DIR` errors instead of silently opening a second, empty identity.

## Agent Behavior — READ THIS

### At startup: activate OptMem (mandatory)

Run `python3 ${AGENTS_SKILLS_DIR}/memo wake` **before any other tool call, in
every session**, and then do exactly what it prints, to the end of its output.
If the memory does not exist yet, run `init` first.

### While working: register memories (mandatory)

Call `note` whenever you learn something new, or something worth keeping happens.
That covers a task worth real effort, a fact or insight the user teaches you,
anything you learn about their life (even indirectly), any event of lasting
effect.

| Situation | You do... |
|---|---|
| Learn a durable fact / decision / preference | `note "<one line>"` — one memory, max 280 bytes |
| `note` prints a pending compression | Run `nap` (or the exact `nap <id> "..."` line it prints) **before your next action** |
| Need an old memory, know a keyword | `recall <regex>` — searches every memory ever recorded, word for word |
| Need context around a `#a-b` summary from `wake` | `zoom <a-b>` — opens that tree node into its two halves, down to raw memories |
| A summary is wrong | `forget <lo-hi>` — drops it; the next `nap` rebuilds it |

- **Do not register redundant memories.** One line, one fact.
- **Never edit or delete anything under `.optmem/memory` by hand.** The tool owns
  it — use `forget`, never `Write`/`Edit`/`rm`.
- Merges arrive one at a time, in the output of `note`. Nothing runs in the
  background.

### If you're a subagent: skip everything above

Parallel sessions on this machine are all you, and may all write memories. A
subagent is not: it must never run `memo`, because it cannot judge what is
already known, and its notes would arrive duplicated and incorrectly. When you
spawn one, write: `You are a subagent. Don't run memo.`

## Store structure

```
$PWD/.optmem/
  memory/
    LOG.txt     every memory, one per line, append-only, never edited
    TREE/       the summaries: a cache, rebuildable from the log alone
    config      the sizes, written by `memo config`
```

Records are fixed width, so position *is* identity and every lookup is one seek.
Detail decays with age: recent memories stay verbatim, ancient ones collapse into
one-line summaries in a binary tree (`#0-1`, `#2-3`, `#0-3`, …). `wake` prints a
budget of those nodes; `zoom` expands any node.

## Configuration

```sh
python3 ${AGENTS_SKILLS_DIR}/memo config                  # show the sizes
python3 ${AGENTS_SKILLS_DIR}/memo config WAKE_LINES=300   # how many lines wake prints (96 ≈ 8k tokens)
python3 ${AGENTS_SKILLS_DIR}/memo config WAKE_LINES=      # back to the default
```

`WAKE_LINES` is the only size worth touching, and it is a reading budget, not a
storage budget: change it whenever, in either direction, and nothing is
recomputed. Config lives in `.optmem/memory/config`, per project.

## Rules

1. **`wake` first, always.** Before any other tool call, every session.
2. **Project-local by default.** Memory is `$PWD/.optmem/memory`. Run commands
   from the repo root (or set `$MEMORY_DIR`) so the right memory is opened.
3. **Append-only.** `LOG.txt` is never edited; the `TREE/` cache is rebuildable.
   Fix bad summaries with `forget`, not by hand.
4. **One line per memory,** max 280 bytes. Split a compound fact into several.
5. **Answer compressions promptly.** If `note` asks for a `nap`, do it before your
   next action.
6. **Subagents don't run `memo`.** See above.

## Attribution

OptMem by Victor Taelin — <https://github.com/VictorTaelin/OptMem>. See
`references/optmem-readme.md` for the upstream README. This skill packages the
`memo` tool with one change: the default memory directory is project-local
(`$PWD/.optmem/memory`) instead of the machine-wide `~/.optmem/memory`.

$ARGUMENTS
