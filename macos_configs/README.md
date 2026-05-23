# macOS Window Manager Configs

This folder keeps the local macOS window-management configs version controlled.

Live config paths:

- `~/.config/yabai/yabairc` -> `macos_configs/yabai/yabairc`
- `~/.config/yabai/move-window-to-space` -> `macos_configs/yabai/move-window-to-space`
- `~/.config/skhd/skhdrc` -> `macos_configs/skhd/skhdrc`

## What We Fixed

macOS's built-in `Tiled windows have margins` setting was already disabled, but visible gaps remained because yabai had its own spacing:

- `top_padding 12`
- `bottom_padding 12`
- `left_padding 12`
- `right_padding 12`
- `window_gap 06`

The fix was to set all of those to `0`. We also disabled yabai window shadows because shadows can visually look like small margins.

The remaining top offset around `32px` is macOS menu-bar reserved space. Normal managed windows cannot occupy that region unless the menu bar is hidden or the app uses native fullscreen.

## yabai vs skhd

`yabai` is the window manager. It controls layout, tiling, padding, gaps, window focus, moving windows between spaces, resizing, floating, and fullscreen behavior.

`skhd` is the hotkey daemon. It listens for keyboard shortcuts and runs commands, often `yabai -m ...` commands.

In this setup:

- `yabai/yabairc` defines the window behavior.
- `skhd/skhdrc` defines keyboard shortcuts.
- `skhd` calls `yabai` commands when shortcuts are pressed.

Example:

```sh
alt - h : yabai -m window --focus west
```

That means: when `Alt+h` is pressed, `skhd` runs a yabai command to focus the window to the west.

## Reloading

Apply current yabai spacing without restarting:

```sh
yabai -m config top_padding 0
yabai -m config bottom_padding 0
yabai -m config left_padding 0
yabai -m config right_padding 0
yabai -m config window_gap 0
```

If using launch services, restart the daemons with whatever mechanism installed them. This machine's yabai install is not managed by `brew services`.
