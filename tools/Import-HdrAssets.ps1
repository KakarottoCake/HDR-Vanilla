param(
    [Parameter(Mandatory = $true)]
    [string]$ArchivePath,

    [string]$DestinationRoot = (Join-Path $PSScriptRoot '..\package'),

    [switch]$Replace
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$expectedArchiveSha256 = 'b7ca9333b0309f0f0893051d68b456a1bb3a8792a0eb1a16df9856d010ac7232'
$stageSourcePrefix = 'ultimate/mods/hdr-stages/'
$stageDestinationRelative = 'ultimate/mods/hdr-vanilla-stages'
$uiSourcePrefix = 'ultimate/mods/hdr-assets/'
$uiDestinationRelative = 'ultimate/mods/hdr-vanilla-ui'
$stageAltsSource = 'ultimate/stage-alts/Hashes_all'
$stagePluginSources = @(
    'atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_alts.nro',
    'atmosphere/contents/01006A800016E000/romfs/skyline/plugins/libstage_config.nro'
)

# These are the HDR UI archives that do not reference HDR mechanics, custom
# moves, the Element selector, HDR game modes, online changes, or HDR branding.
$uiWhitelist = @(
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

function Resolve-SafeTarget {
    param(
        [Parameter(Mandatory = $true)][string]$Root,
        [Parameter(Mandatory = $true)][string]$RelativePath
    )

    $target = [System.IO.Path]::GetFullPath((Join-Path $Root $RelativePath))
    $rootWithSeparator = $Root.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar
    if (-not $target.StartsWith($rootWithSeparator, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Archive entry escapes the package root: $RelativePath"
    }
    return $target
}

function Extract-ZipEntry {
    param(
        [Parameter(Mandatory = $true)]$Entry,
        [Parameter(Mandatory = $true)][string]$Target
    )

    $parent = Split-Path -Parent $Target
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
    [System.IO.Compression.ZipFileExtensions]::ExtractToFile($Entry, $Target, $true)
}

$archive = (Resolve-Path -LiteralPath $ArchivePath).Path
$destination = [System.IO.Path]::GetFullPath($DestinationRoot)
$repositoryRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$packageBoundary = $repositoryRoot.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar

if (-not $destination.StartsWith($packageBoundary, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "DestinationRoot must remain inside the repository: $destination"
}

$actualHash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($actualHash -ne $expectedArchiveSha256) {
    throw "Unexpected archive hash. Expected $expectedArchiveSha256 but found $actualHash."
}

$generatedRoots = @(
    (Join-Path $destination $stageDestinationRelative),
    (Join-Path $destination $uiDestinationRelative),
    (Join-Path $destination 'ultimate/stage-alts'),
    (Join-Path $destination $stagePluginSources[0]),
    (Join-Path $destination $stagePluginSources[1])
)

$existingRoots = @($generatedRoots | Where-Object { Test-Path -LiteralPath $_ })
if ($existingRoots.Count -gt 0 -and -not $Replace) {
    throw "Generated package content already exists. Re-run with -Replace to rebuild it."
}

if ($Replace) {
    foreach ($root in $existingRoots) {
        $resolved = [System.IO.Path]::GetFullPath($root)
        if (-not $resolved.StartsWith($destination.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "Refusing to remove a path outside the package root: $resolved"
        }
        Remove-Item -LiteralPath $resolved -Recurse -Force
    }
}

Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::OpenRead($archive)
$stageCount = 0
$uiCount = 0
$stagePluginCount = 0

try {
    foreach ($entry in $zip.Entries) {
        if ($entry.FullName.EndsWith('/')) {
            continue
        }

        if ($entry.FullName.StartsWith($stageSourcePrefix, [System.StringComparison]::Ordinal)) {
            $relative = $entry.FullName.Substring($stageSourcePrefix.Length)
            $target = Resolve-SafeTarget -Root $destination -RelativePath (Join-Path $stageDestinationRelative $relative)
            Extract-ZipEntry -Entry $entry -Target $target
            $stageCount++
            continue
        }

        if ($entry.FullName.StartsWith($uiSourcePrefix, [System.StringComparison]::Ordinal)) {
            $relative = $entry.FullName.Substring($uiSourcePrefix.Length)
            if ($uiWhitelist -contains $relative) {
                $target = Resolve-SafeTarget -Root $destination -RelativePath (Join-Path $uiDestinationRelative $relative)
                Extract-ZipEntry -Entry $entry -Target $target
                $uiCount++
            }
            continue
        }

        if ($entry.FullName -eq $stageAltsSource) {
            $target = Resolve-SafeTarget -Root $destination -RelativePath $stageAltsSource
            Extract-ZipEntry -Entry $entry -Target $target
            continue
        }

        if ($stagePluginSources -contains $entry.FullName) {
            $target = Resolve-SafeTarget -Root $destination -RelativePath $entry.FullName
            Extract-ZipEntry -Entry $entry -Target $target
            $stagePluginCount++
        }
    }
}
finally {
    $zip.Dispose()
}

$overridesRoot = Join-Path $repositoryRoot 'overrides'
Get-ChildItem -LiteralPath $overridesRoot -Recurse -File | ForEach-Object {
    $relative = [System.IO.Path]::GetRelativePath($overridesRoot, $_.FullName)
    $target = Resolve-SafeTarget -Root $destination -RelativePath $relative
    $parent = Split-Path -Parent $target
    New-Item -ItemType Directory -Force -Path $parent | Out-Null
    Copy-Item -LiteralPath $_.FullName -Destination $target -Force
}

Write-Output "Imported $stageCount stage files and $uiCount neutral UI files from HDR v0.49.11."
Write-Output "Imported $stagePluginCount stage-support plugins; no fighter or general gameplay plugin was imported."
Write-Output "Package root: $destination"
