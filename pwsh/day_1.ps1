#!/usr/bin/env pwsh

Param (
	[string]$FileName
)

if ($FileName -eq "") {
	$FileName =  "../day_1/input.txt"
}

[UInt32[]] $Lines = (Get-Content $FileName).Split("\n")
[UInt32]$Count = 0

for ($idx = 1; $idx -lt $Lines.Length; $idx++) {
	if ($Lines[$idx] -gt $Lines[$idx - 1]) {
		$Count++
	}
}

Write-Output "Part 1: $Count"

[UInt32]$Count = 0
[UInt32]$PreviousSum = [UInt32]::MaxValue

for ($idx = 0; ($idx + 2) -lt $Lines.Length; $idx++) {
	$Sum = $Lines[$idx] + $Lines[$idx + 1] + $Lines[$idx + 2]
	if ($Sum -gt $PreviousSum) {
		$Count++
	}
	$PreviousSum = $Sum
}

Write-Output "Part 2: $Count"
