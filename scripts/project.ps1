[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [ValidateSet("help", "setup", "fmt", "check", "test", "build", "release", "run", "clean", "ci", "install", "uninstall")]
    [string]$Task = "help",

    [Parameter(Position = 1, ValueFromRemainingArguments = $true)]
    [string[]]$TaskArgs
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    function Invoke-Cargo {
        param([Parameter(Mandatory = $true)][string[]]$Arguments)
        & cargo @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE."
        }
    }

    switch ($Task) {
        "help" {
            Write-Host "Forge project tasks:"
            Write-Host "  .\scripts\project.ps1 setup"
            Write-Host "  .\scripts\project.ps1 fmt"
            Write-Host "  .\scripts\project.ps1 check"
            Write-Host "  .\scripts\project.ps1 test"
            Write-Host "  .\scripts\project.ps1 build"
            Write-Host "  .\scripts\project.ps1 release"
            Write-Host "  .\scripts\project.ps1 run -- version"
            Write-Host "  .\scripts\project.ps1 clean"
            Write-Host "  .\scripts\project.ps1 ci"
            Write-Host "  .\scripts\project.ps1 install"
            Write-Host "  .\scripts\project.ps1 uninstall"
        }
        "setup" { Invoke-Cargo @("fetch") }
        "fmt" { Invoke-Cargo @("fmt", "--all", "--", "--check") }
        "check" { Invoke-Cargo @("check", "--all-targets", "--all-features") }
        "test" { Invoke-Cargo @("test", "--all-targets", "--all-features") }
        "build" { Invoke-Cargo @("build") }
        "release" { Invoke-Cargo @("build", "--release") }
        "run" {
            Invoke-Cargo (@("run", "--") + $TaskArgs)
        }
        "clean" { Invoke-Cargo @("clean") }
        "ci" {
            Invoke-Cargo @("fmt", "--all", "--", "--check")
            Invoke-Cargo @("check", "--all-targets", "--all-features")
            Invoke-Cargo @("test", "--all-targets", "--all-features")
            Invoke-Cargo @("build", "--release")
        }
        "install" {
            & (Join-Path $PSScriptRoot "install.ps1") @TaskArgs
            if ($LASTEXITCODE -ne 0) {
                throw "The install task failed with exit code $LASTEXITCODE."
            }
        }
        "uninstall" {
            & (Join-Path $PSScriptRoot "uninstall.ps1") @TaskArgs
            if ($LASTEXITCODE -ne 0) {
                throw "The uninstall task failed with exit code $LASTEXITCODE."
            }
        }
    }
}
finally {
    Pop-Location
}
