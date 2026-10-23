$os = Get-CimInstance Win32_OperatingSystem | Select-Object -First 1
$cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
$memory = Get-CimInstance Win32_ComputerSystem | Select-Object -ExpandProperty TotalPhysicalMemory
$memoryGb = [math]::Round(($memory / 1GB), 2)

Write-Output "OS: $($os.Caption)"
Write-Output "Version: $($os.Version)"
Write-Output "Architecture: $($env:PROCESSOR_ARCHITECTURE)"
Write-Output "Processor: $($cpu.Name)"
Write-Output "Memory: $memoryGb GB"
Write-Output "Computer: $($env:COMPUTERNAME)"
exit 0
