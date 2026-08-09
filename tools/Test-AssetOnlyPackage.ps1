param(
    [string]$PackageRoot = (Join-Path $PSScriptRoot '..\package')
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$root = [System.IO.Path]::GetFullPath($PackageRoot)
$stageRoot = Join-Path $root 'ultimate/mods/hdr-vanilla-stages'
$uiRoot = Join-Path $root 'ultimate/mods/hdr-vanilla-ui'
$stageAltsFile = Join-Path $root 'ultimate/stage-alts/Hashes_all'
$stageAltsPlugin = Join-Path $root 'atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_alts.nro'
$stageConfigPlugin = Join-Path $root 'atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_config.nro'

foreach ($required in @($stageRoot, $uiRoot, $stageAltsFile, $stageAltsPlugin, $stageConfigPlugin)) {
    if (-not (Test-Path -LiteralPath $required)) {
        throw "Required package content is missing: $required"
    }
}

$allFiles = @(Get-ChildItem -LiteralPath $root -Recurse -File)
$stageFiles = @(Get-ChildItem -LiteralPath $stageRoot -Recurse -File)
$uiFiles = @(Get-ChildItem -LiteralPath $uiRoot -Recurse -File)

if ($stageFiles.Count -ne 9073) {
    throw "Expected 9,073 stage files from HDR v0.49.11; found $($stageFiles.Count)."
}

if ($uiFiles.Count -ne 10) {
    throw "Expected 10 cleaned UI files including metadata; found $($uiFiles.Count)."
}

$allowedNroFiles = @(
    [System.IO.Path]::GetFullPath($stageAltsPlugin),
    [System.IO.Path]::GetFullPath($stageConfigPlugin)
)
$nroFiles = @($allFiles | Where-Object { $_.Extension -ieq '.nro' })
$unexpectedNroFiles = @($nroFiles | Where-Object { $allowedNroFiles -notcontains $_.FullName })
if ($unexpectedNroFiles.Count -gt 0 -or $nroFiles.Count -ne 2) {
    throw "Unexpected executable plugin content was found: $($unexpectedNroFiles.FullName -join ', ')"
}

$forbiddenExtensions = @('.npdm', '.lua', '.lc', '.rs')
$forbiddenFiles = @($allFiles | Where-Object { $forbiddenExtensions -contains $_.Extension.ToLowerInvariant() })
if ($forbiddenFiles.Count -gt 0) {
    throw "Unapproved executable or scripted game code was found in the package: $($forbiddenFiles.FullName -join ', ')"
}

$forbiddenSegments = @('\fighter\', '\item\', '\effect\fighter\')
$forbiddenPaths = @($allFiles | Where-Object {
    $normalized = $_.FullName.ToLowerInvariant()
    @($forbiddenSegments | Where-Object { $normalized.Contains($_) }).Count -gt 0
})
if ($forbiddenPaths.Count -gt 0) {
    throw "Gameplay or executable paths were found in the package: $($forbiddenPaths.FullName -join ', ')"
}

$forbiddenUiFragments = @(
    '\message\',
    '\replace\skill\',
    '\replace_patch\skill\',
    '\layout\menu\online_',
    '\layout\menu\title\',
    '\layout\menu\main_menu\',
    '\layout\menu\chara_select_base\',
    '\layout\menu\rule_edit\',
    '\layout\menu\rule_select\',
    '\layout\system\matchup\',
    '\layout\info\info_skill_list\',
    '\layout\info\info_melee\'
)

$badUi = @($uiFiles | Where-Object {
    $normalized = $_.FullName.ToLowerInvariant()
    @($forbiddenUiFragments | Where-Object { $normalized.Contains($_) }).Count -gt 0
})
if ($badUi.Count -gt 0) {
    throw "HDR-only UI content was found in the cleaned UI pack: $($badUi.FullName -join ', ')"
}

$sizeBytes = ($allFiles | Measure-Object Length -Sum).Sum
[pscustomobject]@{
    Result = 'PASS'
    TotalFiles = $allFiles.Count
    StageFiles = $stageFiles.Count
    UiFiles = $uiFiles.Count
    PackageSizeGB = [math]::Round($sizeBytes / 1GB, 2)
    StageSupportPlugins = $nroFiles.Count
    ExecutableGameplayCode = 0
    FighterFiles = 0
    OnlineUiFiles = 0
} | Format-List
