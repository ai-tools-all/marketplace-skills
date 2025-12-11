# Claude Code Marketplace (abeeshake)
- Community-driven Claude Code commands and agents maintained in this repo.
- Install the marketplace, browse available plugins, and add the ones you want.

## Quick start
- Add the marketplace (GitHub):  
  `/plugin marketplace add abhishek/marketplace_of_abeeshake`
- Add the marketplace locally from a clone:  
  `/plugin marketplace add ./marketplace_of_abeeshake`
- Browse and install from the UI:  
  `/plugin`
- Install directly by name (example):  
  `/plugin install bug-detective@marketplace-of-abeeshake`

## Featured plugins
- `bug-detective` — systematic debugging assistant with stepwise troubleshooting.

## Repo layout
- `.claude-plugin/marketplace.json` — marketplace manifest (fill with your marketplace name and plugin entries).
- `plugins/bug-detective/` — plugin folder with manifest and command definition.
- `docs/2025-12-11-how-to-plugin-guide.md` — how to create and test plugins.
- `docs/2025-12-11-plugin-reference-detailed-manual.md` — reference copy for plugin structure and workflows.

## Using a plugin
- After install, run its command from Claude Code (e.g., `/bug-detective`).
- Use `/help` to confirm new commands are registered.
- Manage lifecycle with `/plugin enable|disable|uninstall <name>@marketplace-of-abeeshake`.
