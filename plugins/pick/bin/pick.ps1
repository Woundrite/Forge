param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Candidates
)

if ($Candidates.Count -eq 0) {
    Write-Error "At least one candidate is required."
    exit 2
}

$choice = Get-Random -InputObject $Candidates
Write-Output $choice
exit 0
