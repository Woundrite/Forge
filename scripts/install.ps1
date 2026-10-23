[CmdletBinding()]
param(
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA "Forge"),
    [switch]$SkipBuild,
    [switch]$AddToPath,
    [switch]$InstallShellIntegration
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
$installDir = [System.IO.Path]::GetFullPath($InstallDir)
$binary = Join-Path $repoRoot "target\release\forge.exe"
$markerPath = Join-Path $installDir ".forge-install.json"

if (-not $SkipBuild) {
    Write-Host "Building Forge in release mode..."
    cargo build --release --manifest-path (Join-Path $repoRoot "Cargo.toml")
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo release build failed with exit code $LASTEXITCODE."
    }
}

if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw "Forge binary was not found at '$binary'. Build the project first or omit -SkipBuild."
}

New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Copy-Item -LiteralPath $binary -Destination (Join-Path $installDir "forge.exe") -Force

$installedPlugins = @()
$sourcePlugins = Join-Path $repoRoot "plugins"
if (Test-Path -LiteralPath $sourcePlugins -PathType Container) {
    Get-ChildItem -LiteralPath $sourcePlugins -Directory | ForEach-Object {
        if ($_.Name -eq "README.md") {
            return
        }
        $destination = Join-Path $installDir "plugins\$($_.Name)"
        Copy-Item -LiteralPath $_.FullName -Destination $destination -Recurse -Force
        $installedPlugins += $destination
    }
}

$installedFiles = @(
    (Join-Path $installDir "forge.exe")
)

if ($AddToPath) {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $pathEntries = @($userPath -split ";" | Where-Object { $_ })
    if ($pathEntries -notcontains $installDir) {
        [Environment]::SetEnvironmentVariable("Path", (($pathEntries + $installDir) -join ";"), "User")
        Write-Host "Added '$installDir' to the user PATH. Open a new terminal to use it."
    }
}

if ($InstallShellIntegration) {
    $moduleDir = Join-Path ([Environment]::GetFolderPath("MyDocuments")) "PowerShell\Modules\Forge"
    New-Item -ItemType Directory -Force -Path $moduleDir | Out-Null
    Copy-Item -LiteralPath (Join-Path $repoRoot "integrations\powershell\Forge.psm1") `
        -Destination (Join-Path $moduleDir "Forge.psm1") -Force
    Write-Host "Installed the PowerShell module. Import it with: Import-Module Forge"
}

$configPath = & $binary config path
if ($LASTEXITCODE -ne 0) {
    throw "Could not determine the Forge configuration path."
}
$configPath = ($configPath | Select-Object -Last 1).Trim()
& $binary config add-plugin-dir (Join-Path $installDir "plugins")
if ($LASTEXITCODE -ne 0) {
    throw "Could not register the installed plugin directory."
}

$metadata = [ordered]@{
    install_dir = $installDir
    installed_at = (Get-Date).ToUniversalTime().ToString("o")
    files = $installedFiles
    plugin_directories = $installedPlugins
    config_path = $configPath
    path_entry_added = [bool]$AddToPath
    powershell_module = [bool]$InstallShellIntegration
}
$metadata | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath $markerPath -Encoding UTF8

Write-Host "Forge installed to '$installDir'."
Write-Host "Run '$installDir\forge.exe help' to verify the installation."
