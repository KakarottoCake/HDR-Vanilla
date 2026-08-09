# HDR Vanilla Assets

An offline-focused, asset-only fork of HewDraw Remix that keeps HDR's stages and a cleaned subset of its interface while preserving vanilla Super Smash Bros. Ultimate gameplay.

This project does **not** include movesets, fighter balance, custom mechanics, input or latency changes, online changes, or an HDR gameplay plugin. It includes only the two executable stage-support dependencies needed for alternate stages and configurable hazards.

## Pinned upstream release

- HewDraw Remix source tag: `v0.49.11`
- HDR release package: `v0.49.11`
- Release package SHA-256: `b7ca9333b0309f0f0893051d68b456a1bb3a8792a0eb1a16df9856d010ac7232`

## Build the local package

Download `switch-package.zip` from the official HDR `v0.49.11` release, then run:

```powershell
.\tools\Import-HdrAssets.ps1 -ArchivePath .\switch-package.zip
.\tools\Test-AssetOnlyPackage.ps1
```

The generated package is written to `package/`. Rebuild with `-Replace` when the generated folders already exist.

## Installation expectations

Copy the contents of `package/` to the SD-card root used by your existing Smash modding setup. ARCropolis is required. Compatible `stage_alts` and `stage_config` plugins are bundled from the pinned HDR release so alternate stages and configurable hazards work as intended.

See [ASSET_SCOPE.md](ASSET_SCOPE.md) for the inclusion policy, [OMITTED_INVENTORY.md](OMITTED_INVENTORY.md) for the complete omission list, and [TESTING.md](TESTING.md) for the offline verification matrix.

## Redistribution

The import script generates a local package from the official HDR release so this repository does not need to republish third-party UI and stage binaries. Before publishing generated assets, obtain permission from the HDR team and the original authors identified by them. Preserve `stage_credits.txt` and all required author/license notices.

This project is not affiliated with Nintendo, Sora Ltd., Bandai Namco, or the HDR development team. It does not provide game files and does not support piracy.
