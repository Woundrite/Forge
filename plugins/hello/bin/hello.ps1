param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Names
)

if ($Names.Count -eq 0) {
    Write-Output "Hello, Forge!"
    exit 0
}

$joined = $Names -join ' '
Write-Output "Hello, $joined!"
exit 0
