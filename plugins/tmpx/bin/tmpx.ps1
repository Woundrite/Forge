param(
    [string]$Ttl = "",
    [switch]$Help
)

if ($Help) {
    Write-Output "Usage: forge tmpx [--ttl 2h]"
    exit 0
}

$tempRoot = if ($env:TEMP) { $env:TEMP } else { [System.IO.Path]::GetTempPath() }
$stamp = Get-Date -Format yyyyMMdd-HHmmss
$id = [System.Guid]::NewGuid().ToString('N')
$target = Join-Path $tempRoot "forge-$stamp-$id"
New-Item -ItemType Directory -Path $target | Out-Null

if ($Ttl) {
    if ($Ttl -notmatch '^(\d+)([smhd])$') {
        Write-Error "Invalid TTL format: $Ttl. Expected values like 2h or 90m."
        exit 2
    }
    Write-Output "Created: $target"
    Write-Output "TTL: $Ttl"
    exit 0
}

Write-Output $target
exit 0
