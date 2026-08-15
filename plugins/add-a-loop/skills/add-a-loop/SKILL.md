---
name: add-a-loop
description: Add a recurring reminder or background job ("loop") to the qdb loops subsystem from the CLI. Use whenever the request is "schedule a reminder", "add a cron/daily/recurring job", "run X every N hours", "remind me at 9:30", or any recurring-schedule task against `qdb loops`. Covers the serve → add → list flow, the four schedule forms, kinds, and gateway delivery.
---

# Add a loop

A **loop** is a named, recurring schedule stored in the loops DB. When it comes due,
the runtime enqueues a one-shot job (River) of the loop's `kind`, which fans out to
the configured outgoing gateways. Adding one is a CLI call to a **running service** —
the CLI is a thin client over unix-socket/TCP; nothing schedules unless `serve` is up.

## The flow

```sh
# 0. Preflight — the whole skill is a thin client over the `qdb` CLI.
#    If it's not on PATH, STOP and tell the user; don't attempt the rest.
command -v qdb >/dev/null 2>&1 || { echo "qdb CLI not found on PATH — install it first"; exit 1; }

# 1. Runtime must be running (holds the River queue + fires due loops).
#    Default addr is :6777, or unix://<catalog-dir>/... — resolved from the open DB.
qdb loops serve            # foreground; run in tmux/& to keep it up

# 2. Add the loop — PREFER --rrule (most explicit, no ambiguity, survives DST sanely).
qdb loops add standup --rrule "FREQ=DAILY;BYHOUR=9;BYMINUTE=30"
qdb loops add monday  --rrule "FREQ=WEEKLY;BYDAY=MO;BYHOUR=9"
qdb loops add report  --rrule "FREQ=WEEKLY;BYDAY=FR;BYHOUR=17"
# fall back to the others only when rrule can't express it:
qdb loops add poll    --every 6h                # sub-daily fixed interval

# 3. Verify — shows Name, NextRun, and the rrule/cron it resolved to.
qdb loops list
```

If the service isn't up, the client call errors on connect — start `serve` first.
Point at a non-default runtime with `--listen unix://path.sock` or `--listen :6777`
on any subcommand.

## Schedule forms (pick one — prefer rrule)

**Default to `--rrule`.** It's the most expressive and unambiguous; `--at` is just sugar
that gets converted into an rrule anyway. Reach for the others only when rrule genuinely
can't say it (e.g. a sub-daily fixed interval → `--every`).

| Flag | Example | When |
|------|---------|------|
| `--rrule` | `--rrule "FREQ=DAILY;BYHOUR=9;BYMINUTE=30"` | **preferred** — any calendar schedule |
| `--every` | `--every 6h` | fixed interval with no clock anchor (sub-daily) |
| `--at` | `--at 9:30`, `--at 5:00pm` | quick daily clock; expands to the rrule above |
| `--cron` | `--cron "30 9 * * *"` | only if you're pasting an existing cron line |

`add` rejects the call if none is given (`need --rrule, --cron, --at 9:30, or --every 24h`).
Precedence when several are set: rrule/cron win over `--at`, which wins over `--every`.

## Kind and message

- `--kind` — `remind` (default), `ping`, or `sql`. This is the job kind enqueued on fire.
- `--message` — the reminder text; defaults to the loop name. Stored as the job payload
  (`{"title": "..."}`).

```sh
qdb loops add backup --every 12h --kind sql --message "VACUUM;"
```

## Delivery — gateways

A fired loop only reaches you if an outgoing gateway is configured. Check with:

```sh
qdb loops gateways     # lists configured gateways, no secrets
qdb loops send "test"  # fire a message through all gateways right now
```

Gateways come from env: `TELEGRAM_BOT_TOKEN`+`TELEGRAM_CHAT_ID`, `SLACK_WEBHOOK_URL`,
and/or `LOOPS_WEBHOOK_URL`. With none set, `send` errors and fired reminders only land
in the inbox (`qdb loops inbox`). Never print these token values.

## Related one-shots (not loops)

- `qdb loops remind <title> [--in 20m | --at <RFC3339>]` — single future reminder.
- `qdb loops enqueue <kind> [payload]` — run a job once, now.

## Checklist

- [ ] `qdb` CLI is on PATH (`command -v qdb`) — if not, tell the user and stop
- [ ] `qdb loops serve` is running (default `:6777` or the DB's unix socket)
- [ ] Exactly one of `--at` / `--cron` / `--rrule` / `--every`
- [ ] `--kind` and `--message` set if not defaulting to `remind` / the loop name
- [ ] `qdb loops list` shows the expected NextRun
- [ ] A gateway is configured (`qdb loops gateways`) or you accept inbox-only delivery
