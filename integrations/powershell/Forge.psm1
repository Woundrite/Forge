function forge {
    param(
        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Arguments
    )

    & forge.exe @Arguments
    if ($LASTEXITCODE -ne 0) {
        return
    }
}

Export-ModuleMember -Function forge
