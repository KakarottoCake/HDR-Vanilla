# Offline test plan

The repository can prove that the package is asset-only, but final in-game checks require a modded Switch or a legally configured local emulator with the user's own game dump.

## Automated package checks

- Exact HDR v0.49.11 source archive hash.
- Exact stage and UI file counts.
- Exactly two executable plugins: `stage_alts` and `stage_config`.
- No HDR gameplay plugin, scripts, fighter files, item files, input patches, or other Atmosphere executables.
- No online, Element selector, HDR move, skill, matchup, title, rules, or main-menu UI archives.

Run:

```powershell
.\tools\Test-AssetOnlyPackage.ps1
```

## Required offline in-game checks

- Boot to title and reach the main menu.
- Open local Smash, Squad Strike, Tournament, Special Smash, and Training.
- Open character select with every player count and CPU configuration.
- Select every fighter and costume; verify no Element selector or HDR move text appears.
- Open rule select/edit, controller options, pause, camera, and results screens.
- Load every retained stage with hazards both on and off where supported.
- Verify Battlefield and Omega forms, stage alts, music selection, camera bounds, collisions, blast zones, ledges, moving platforms, hazards, respawn points, and final-smash camera behavior.
- Verify items, Assist Trophies, Poké Balls, Final Smashes, stamina, spirits-off rules, and eight-player local play remain vanilla.
- Verify Training reset, frame advance, CPU controls, and combo counter screens.
- Complete several local matches and return cleanly to character and stage select.

Online is deliberately outside the supported and tested scope.
