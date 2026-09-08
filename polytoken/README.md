# Warm Burnout for Polytoken

Your terminal harness burnout treatment, now with consistent damage across the whole TUI.

## Install

Copy the theme file to your Polytoken themes directory:

```sh
# Linux
mkdir -p ~/.config/polytoken/themes
cp warm-burnout.yaml ~/.config/polytoken/themes/

# macOS
mkdir -p ~/Library/Application\ Support/polytoken/themes
cp warm-burnout.yaml ~/Library/Application\ Support/polytoken/themes/
```

## Select

Set the theme in your Polytoken config:

```toml
[tui]
theme-file = "warm-burnout"
```

Or run `/theme` inside the TUI and pick `Warm Burnout` for a session-local preview.

## Verify It Landed

```sh
polytoken theme list
polytoken theme validate --all
```

## Both Variants Included

One file, both variants. `dark` and `light` token sections point at a shared palette, so the two variants cannot drift apart.

## Palette

Both variants derive from the canonical Warm Burnout palette defined in the root [`AGENTS.md`](../AGENTS.md). Contrast audited, deadlines remain.
