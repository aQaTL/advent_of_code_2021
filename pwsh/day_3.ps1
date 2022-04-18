[string[]]$Lines =  (Get-Content "../day_3/input.txt").Split("\n")

[string]$GammaRate = ""
[uint32]$LineLength = $Lines[0].Length

for ($BitIdx = 0; $BitIdx -lt $LineLength; $BitIdx++) {
	[int32]$Ones = 0
	[int32]$Zeros = 0

	$Lines 
		| %{ $_[$BitIdx] -eq '0' ? $Zeros++ : $Ones++ } 
		| Out-Null

	$GammaRate += $Ones -gt $Zeros ? "1" : "0"
}

[uint32]$Epsilon =
	([char[]]$GammaRate
		| ForEach-Object { $_ -eq '1' ? "0" : "1" }) `
	-Join ""
	| %{ [Convert]::ToUInt32($_, 2) }

[uint32]$GammaRate = [Convert]::ToUInt32($GammaRate, 2)

Write-Output "Part 1: $($GammaRate * $Epsilon)"

[string[]]$LinesMostCommon, [string[]]$LinesLeastCommon = $Lines, $Lines

for ($BitIdx = 0; $BitIdx -lt $LineLength; $BitIdx++) {	
	if ($LinesMostCommon.Length -gt 1) {
		[int32]$Ones, [int32]$Zeros = 0, 0

		$LinesMostCommon 
			| %{ $_[$BitIdx] -eq '0' ? $Zeros++ : $Ones++ } 
			| Out-Null

		[char]$KeepMostCommon = $Ones -ge $Zeros ? '1' : '0'

		[string[]]$LinesMostCommon = $LinesMostCommon
			| Where-Object { $_[$BitIdx] -eq $KeepMostCommon }
	}
	
	if ($LinesLeastCommon.Length -gt 1) {
		[int32]$Ones, [int32]$Zeros = 0, 0

		$LinesLeastCommon 
			| %{ $_[$BitIdx] -eq '0' ? $Zeros++ : $Ones++ } 
			| Out-Null

		[char]$KeepLeastCommon = $Zeros -le $Ones ? '0' : '1'

		[string[]]$LinesLeastCommon = $LinesLeastCommon
			| Where-Object { $_[$BitIdx] -eq $KeepLeastCommon }
	}
}

[uint32]$OxygenGeneratorRating = [Convert]::ToUInt32($LinesMostCommon[0], 2)
[uint32]$CO2ScrubberRating = [Convert]::ToUInt32($LinesLeastCommon[0], 2)

Write-Output "Part 2: $($OxygenGeneratorRating * $CO2ScrubberRating)"
