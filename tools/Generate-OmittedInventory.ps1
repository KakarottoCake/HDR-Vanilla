param(
    [Parameter(Mandatory = $true)]
    [string]$ManifestPath,

    [string]$OutputPath = (Join-Path $PSScriptRoot '..\OMITTED_INVENTORY.md')
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$manifest = Get-Content -LiteralPath $ManifestPath -Raw | ConvertFrom-Json
$retainedUi = @(
    'ui/layout/info/info_1on1/info_1on1/layout.arc',
    'ui/layout/info/info_center/info_center/layout.arc',
    'ui/layout/info/info_pause/info_pause/layout.arc',
    'ui/layout/menu/main_menu_footer/main_menu_footer/layout.arc',
    'ui/layout/menu/chara_select/chara_select/layout.arc',
    'ui/layout/menu/option_button_subwindow/option_button_subwindow/layout.arc',
    'ui/layout/menu/pallet_menu/pallet_menu/layout.arc',
    'ui/layout/system/loading/loading/layout.arc',
    'ui/param/common/common_ui_base_param.prc'
)
$allowedPlugins = @(
    '/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_alts.nro',
    '/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_config.nro'
)

$lines = [System.Collections.Generic.List[string]]::new()
$lines.Add('# Omitted inventory')
$lines.Add('')
$lines.Add('Generated from the pinned HDR v0.49.11 `content_hashes.json` manifest. The listed paths are omitted from the generated package; whole directory prefixes are used where the omission is complete.')
$lines.Add('')
$lines.Add('## Retained')
$lines.Add('')
$lines.Add('- All 9,073 files under `/ultimate/mods/hdr-stages/`.')
$lines.Add('- `/ultimate/stage-alts/Hashes_all`.')
$lines.Add('- `/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_alts.nro`.')
$lines.Add('- `/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_config.nro`.')
$lines.Add('- The nine neutral UI paths listed in `tools/Import-HdrAssets.ps1`.')
$lines.Add('')

$lines.Add('## Entire HDR gameplay mod omitted')
$lines.Add('')
$lines.Add('- Entire `/ultimate/mods/hdr/` tree: 303 files.')
$lines.Add('  - 270 fighter files.')
$lines.Add('  - 23 HDR UI files.')
$lines.Add('  - 7 item files.')
$lines.Add('  - 1 parameter file.')
$lines.Add('  - `config.json` and `plugin.nro`.')
$lines.Add('')
$lines.Add('### Omitted files under `/ultimate/mods/hdr/ui/`')
$lines.Add('')
$manifest | Where-Object { $_.path -like '/ultimate/mods/hdr/ui/*' } | Sort-Object path | ForEach-Object {
    $lines.Add("- $($_.path)")
}
$lines.Add('')

$lines.Add('## HDR asset-pack directories omitted')
$lines.Add('')
$assets = @($manifest | Where-Object { $_.path -like '/ultimate/mods/hdr-assets/*' })
$assets | ForEach-Object {
    $parts = $_.path.TrimStart('/').Split('/')
    if ($parts.Length -ge 5) { $parts[3] } else { '(root file)' }
} | Group-Object | Sort-Object Name | ForEach-Object {
    if ($_.Name -eq '(root file)') {
        $assets | Where-Object { $_.path.TrimStart('/').Split('/').Length -eq 4 } | Sort-Object path | ForEach-Object {
            $lines.Add("- Omitted HDR-assets root file: $($_.path)")
        }
    } elseif ($_.Name -eq 'ui') {
        $lines.Add('- `/ultimate/mods/hdr-assets/ui/`: 174 of 183 files omitted; the nine retained files are the neutral UI whitelist.')
    } else {
        $lines.Add("- Entire `/ultimate/mods/hdr-assets/$($_.Name)/` tree: $($_.Count) files.")
    }
}
$lines.Add('')
$lines.Add('## HDR-assets UI files omitted individually')
$lines.Add('')
$omittedUi = @($assets | Where-Object {
    $_.path -like '/ultimate/mods/hdr-assets/ui/*' -and
    $retainedUi -notcontains $_.path.Substring('/ultimate/mods/hdr-assets/'.Length)
} | Select-Object -ExpandProperty path | Sort-Object)
$omittedUi | ForEach-Object { $lines.Add("- $_") }
$lines.Add('')
$lines.Add("Total omitted HDR-assets UI files: $($omittedUi.Count).")
$lines.Add('')

$lines.Add('## Other executable dependencies omitted')
$lines.Add('')
$manifest | Where-Object { $_.path -like '/atmosphere/*' -and $allowedPlugins -notcontains $_.path } | Sort-Object path | ForEach-Object {
    $lines.Add("- $($_.path)")
}
$lines.Add('')

$lines.Add('## Source repository tree removed from this branch')
$lines.Add('')
$lines.Add('These are the complete upstream source/build roots removed from the Git branch because this fork has no gameplay code:')
$lines.Add('')
@(
    '.github/',
    'dynamic/',
    'fighters/',
    'hdr-macros/',
    'romfs/',
    'scripts/',
    'src/',
    'utils/',
    'Cargo.toml',
    'hdr_version.txt',
    'rustfmt.toml'
) | ForEach-Object { $lines.Add("- $_") }
$lines.Add('')
$lines.Add('The source-side UI items omitted with `romfs/` included HDR stage-select Lua, menu Lua, HDR mechanic documentation, fighter/item parameters, and the HUD/menu parameter sources. They were not copied because the requested fork has no game-code changes.')

$output = [System.IO.Path]::GetFullPath($OutputPath)
[System.IO.File]::WriteAllLines($output, $lines)
Write-Output "Wrote $output"
