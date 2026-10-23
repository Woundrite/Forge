[CmdletBinding()]
param(
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA "Forge"),
    [switch]$PurgeData
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$installDir = [System.IO.Path]::GetFullPath($InstallDir)
$markerPath = Join-Path $installDir ".forge-install.json"

if (-not (Test-Path -LiteralPath $markerPath -PathType Leaf)) {
    Write-Warning "No Forge installation marker was found at '$markerPath'. Nothing was removed."
    exit 0
}

$metadata = Get-Content -LiteralPath $markerPath -Raw | ConvertFrom-Json
if ([System.IO.Path]::GetFullPath($metadata.install_dir) -ne $installDir) {
    throw "Installation marker does not belong to '$installDir'."
}

$forgeBinary = Join-Path $installDir "forge.exe"
$pluginDirectory = Join-Path $installDir "plugins"
if (Test-Path -LiteralPath $forgeBinary -PathType Leaf) {
    & $forgeBinary config remove-plugin-dir $pluginDirectory
    if ($LASTEXITCODE -ne 0) {
        throw "Could not unregister the installed plugin directory."
    }
}

foreach ($file in @($metadata.files)) {
    if (Test-Path -LiteralPath $file -PathType Leaf) {
        Remove-Item -LiteralPath $file -Force
    }
}

foreach ($directory in @($metadata.plugin_directories)) {
    if (Test-Path -LiteralPath $directory -PathType Container) {
        Remove-Item -LiteralPath $directory -Recurse -Force
    }
}

if ($metadata.path_entry_added) {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $pathEntries = @($userPath -split ";" | Where-Object { $_ -and $_ -ne $installDir })
    [Environment]::SetEnvironmentVariable("Path", ($pathEntries -join ";"), "User")
}

if ($metadata.powershell_module) {
    $modulePath = Join-Path ([Environment]::GetFolderPath("MyDocuments")) "PowerShell\Modules\Forge"
    if (Test-Path -LiteralPath $modulePath -PathType Container) {
        Remove-Item -LiteralPath $modulePath -Recurse -Force
    }
}

Remove-Item -LiteralPath $markerPath -Force
if (Test-Path -LiteralPath $pluginDirectory -PathType Container) {
    $remainingPlugins = Get-ChildItem -LiteralPath $pluginDirectory -Force
    if (-not $remainingPlugins) {
        Remove-Item -LiteralPath $pluginDirectory -Force
    }
}
if (Test-Path -LiteralPath $installDir -PathType Container) {
    $remaining = Get-ChildItem -LiteralPath $installDir -Force
    if (-not $remaining) {
        Remove-Item -LiteralPath $installDir -Force
    }
}

if ($PurgeData) {
    $configDirectory = Split-Path -Parent ([System.IO.Path]::GetFullPath($metadata.config_path))
    if (Test-Path -LiteralPath $configDirectory -PathType Container) {
        Remove-Item -LiteralPath $configDirectory -Recurse -Force
        Write-Host "Removed Forge configuration and user data from '$configDirectory'."
    }
} else {
    Write-Host "Forge configuration and user data were preserved. Use -PurgeData to remove them explicitly."
}

Write-Host "Forge was uninstalled from '$installDir'."
