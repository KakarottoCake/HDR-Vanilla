# Asset scope

This fork is pinned to HewDraw Remix `v0.49.11`.

## Included

- The complete `hdr-stages` asset pack, renamed to `hdr-vanilla-stages`.
- Stage-select images, names, layouts, sounds, effects, collisions, and stage configuration shipped inside that pack.
- The stage-alts `Hashes_all` data file.
- The unmodified `stage_alts` and `stage_config` support plugins required for alternate-stage loading and configurable stage hazards.
- Neutral HDR interface archives for the in-match HUD, pause UI, loading screen, character-select shell, palette menu, footer, and related common UI styling.

## Excluded

- HDR's gameplay plugin and every executable other than the two explicitly allowed stage-support plugins.
- Fighter, moveset, animation, parameter, item, input, latency, control, physics, and balance changes.
- HDR skill icons, move descriptions, gameplay tips, and mechanic documentation.
- Element-selector UI and its character-select base archive.
- HDR game-mode, rules, main-menu, title, matchup, and branding archives.
- Online-specific UI and online behavior.
- HID-HDR, Smashline, HDR Launcher, and all other executable dependencies from the full HDR package.

## UI whitelist

The import is intentionally whitelist-based. If upstream adds another UI archive, it is excluded until it is audited and explicitly added to `tools/Import-HdrAssets.ps1`.
