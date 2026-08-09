param(
    [string]$PackageRoot = (Join-Path $PSScriptRoot '..\package'),
    [string]$OutputPath = (Join-Path $PSScriptRoot '..\release\hdr-vanilla-assets-v0.1.0.zip')
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$package = (Resolve-Path -LiteralPath $PackageRoot).Path
$output = [System.IO.Path]::GetFullPath($OutputPath)
$repositoryRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$releaseRoot = [System.IO.Path]::GetFullPath((Join-Path $repositoryRoot 'release'))

if (-not $output.StartsWith($releaseRoot.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "OutputPath must remain inside the repository release directory: $output"
}

& (Join-Path $PSScriptRoot 'Test-AssetOnlyPackage.ps1') -PackageRoot $package | Out-Host

New-Item -ItemType Directory -Force -Path $releaseRoot | Out-Null
if (Test-Path -LiteralPath $output) {
    Remove-Item -LiteralPath $output -Force
}

Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::CreateFromDirectory(
    $package,
    $output,
    [System.IO.Compression.CompressionLevel]::Fastest,
    $false
)

$zip = [System.IO.Compression.ZipFile]::OpenRead($output)
try {
    if ($zip.Entries.Count -ne 9086) {
        throw "Release archive contains $($zip.Entries.Count) entries; expected 9,086."
    }
}
finally {
    $zip.Dispose()
}

$hash = (Get-FileHash -LiteralPath $output -Algorithm SHA256).Hash.ToLowerInvariant()
$hashPath = $output + '.sha256'
[System.IO.File]::WriteAllText($hashPath, "$hash  $([System.IO.Path]::GetFileName($output))`n")

[pscustomobject]@{
    Archive = $output
    SizeGB = [math]::Round((Get-Item -LiteralPath $output).Length / 1GB, 2)
    Entries = 9086
    SHA256 = $hash
} | Format-List
