# Polytoken -- Agent Instructions

## Platform Reference

See the root [`AGENTS.md`](../AGENTS.md) for the canonical palette, design principles, and brand rules. Do not duplicate palette tables here.

## Polytoken Theme Format

- Single YAML file with `version: 1`, `title`, `palette`, and `dark`/`light` token sections.
- `palette` holds every color, organized into categories (`text`, `surface`, `semantic`, `chrome`, `reference`, `model_family`, `diff`, `syntax`, `status_bar`, `sidebar`, `inline`, `animation`, `emphasis`). Define each unique color once; everything else references it via `$ref`.
- `dark:` and `light:` sections map token names to style objects (`fg`, `bg`, `b`, `i`, `u`, `s`) whose colors are `$ref` pointers into the palette.
- Both variants live in one file: `warm-burnout.yaml`.
- User themes go in `$XDG_CONFIG_HOME/polytoken/themes/` (Linux) or `~/Library/Application Support/polytoken/themes/` (macOS).
- Validate with `polytoken theme validate polytoken/warm-burnout.yaml` after every edit.

## Role to Canonical Token Mapping

### Core UI

- `body` <- Foreground
- `muted`, `muted_emphasis`, `tool_detail_bar`, `code_punctuation` <- VS Code secondary UI foreground (#ada69c dark, #5c5750 light)
- `dim` <- Activity bar top foreground (dark), indent guide tone (light)
- `accent`, `status_bar_dirty`, `goal_accent` <- Functions amber, not the brand accent (see design decision 1)
- `border` <- widget.border overlay composited to opaque over the app background
- `app_background` <- Canonical background
- `floating_window_background`, `assistant_message_background`, `system_reminder_background` <- VS Code editor widget background
- `user_message_background` <- VS Code input background
- `status_bar_block` <- VS Code sidebar/panel background
- `focus_ring`, `selected`, `prompt_selection` <- Ghostty selection background (neutral by design)

### Semantic

- `error`, `diff_removed` <- Error/invalid token
- `warning` <- Functions amber
- `success`, `diff_added` <- Strings (warm sage green)
- `info`, `slash_mode_foreground` <- Types accent (the one cool color)

### Syntax

`code_keyword` (bold), `code_string`, `code_comment` (italic), `code_literal`, `code_number`, `code_type` (italic), `code_function`, `code_operator`, `code_punctuation` map directly to the canonical token of the same name. Keyword, comment, and type carry the three-tier font styles.

### Chrome Cards

- `facet_chrome` <- Decorators
- `ask_user_question_chrome` <- Member vars
- `subagent_chrome` <- Strings
- `link` <- Functions amber, underlined
- `code_inline` <- Types accent (the one cool color)

### Model Families

Each family gets a warm palette tone; none introduce a second cool hue. `nvidia` keeps the NVIDIA brand green in both variants, same as the shipped rose-pine theme.

## File Naming

- Single file: `warm-burnout.yaml`
- No variant suffix needed: both dark and light are in one file.

## Design Decisions

1. The `accent` token uses functions amber (#ffb454 dark, #924800 light) instead of the brand accent #b8522e. The brand accent fails WCAG AAA as text on the dark background (about 3.7:1) and AA on the light background (about 4.2:1). It stays on badges, buttons, and selection borders in the other platform variants, where it sits on its own fill. In Polytoken the accent is text, so readability wins.
2. `status_bar_dirty` and `status_bar_monitor_bypass_plus` use keywords orange (#ff8f40) rather than ANSI red. The suite restricts blue/red conventions to terminal ANSI and git indicators; a harness status dot is neither.
3. Surfaces come straight from the VS Code variant: cards reuse `editorWidget.background`, the status bar and user prompt cards reuse the sidebar/input backgrounds. Terminal selection backgrounds come from the Ghostty variant. Non-canonical colors are never invented, only lifted from sibling platforms.
4. `command_mode_background` and `slash_mode_background` are opaque tints blended over the app background (red for `!`, desaturated steel for `/`). Slash mode keeps the types accent as its foreground: one cool hue, even in mode labels.
5. Diff colors reuse canonical tokens instead of ANSI red/green. Polytoken renders diffs itself in markdown-style previews, so it follows the suite's syntax mapping rather than the terminal-ANSI convention used by editor diffs.
