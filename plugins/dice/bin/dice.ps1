param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

if ($Args.Count -eq 0) {
    Write-Error "Usage: forge dice <NdS> [keep-highest|keep-lowest]"
    exit 2
}

$notation = $Args[0]
if ($notation -notmatch '^(\d+)d(\d+)(kh|kl)?(\d+)?$') {
    Write-Error "Invalid dice notation: $notation"
    exit 2
}

$rolls = 1
$faces = [int]$Matches[2]
$keep = $Matches[3]
$keepCount = if ($null -ne $Matches[4]) { [int]$Matches[4] } else { $null }

if ($keep -and $keepCount -is [int] -and $keepCount -lt 1) {
    Write-Error "Keep count must be at least 1."
    exit 2
}

$dieCount = [int]$Matches[1]
$results = @()
foreach ($i in 1..$dieCount) {
    $value = Get-Random -Minimum 1 -Maximum ($faces + 1)
    $results += $value
}

if ($keep) {
    $sorted = @($results | Sort-Object)
    if ($keep -eq 'kh') {
        $results = $sorted[-$keepCount..-1]
    } else {
        $results = $sorted[0..($keepCount - 1)]
    }
}

$total = ($results | Measure-Object -Sum).Sum
Write-Output "Rolls: $($results -join ', ')"
Write-Output "Total: $total"
exit 0
