Param (
	[string]$FileName
)

if ($FileName -eq "") {
	$FileName = "../day_2/input.txt"
}

[string[]] $Lines = (Get-Content $FileName).Split("\n")

[Int32]$HorizontalPos = 0
[Int32]$Depth = 0

foreach ($Line in $Lines) {
	switch -Regex ($Line) {
		"^forward (\d+)$" {
			[Int32]$Val = $Matches[1]
			$HorizontalPos += $Val
			Break;
		}
		"^down (\d+)$" {
			[Int32]$Val = $Matches[1]
			$Depth += $Val
			Break;
		}
		"^up (\d+)$" {
			[Int32]$Val = $Matches[1]
			$Depth -= $Val
			Break;
		}
	}
}

Write-Output "Part 1: $($HorizontalPos * $Depth)"

[Int32]$HorizontalPos = 0
[Int32]$Depth = 0
[Int32]$Aim = 0

foreach ($Line in $Lines) {
	switch -Regex ($Line) {
		"^forward (\d+)$" {
			[Int32]$Val = $Matches[1]
			$HorizontalPos += $Val
			$Depth += $Aim * $Val
			Break;
		}
		"^down (\d+)$" {
			[Int32]$Val = $Matches[1]
			$Aim += $Val
			Break;
		}
		"^up (\d+)$" {
			[Int32]$Val = $Matches[1]
			$Aim -= $Val
			Break;
		}
	}
}

Write-Output "Part 2: $($HorizontalPos * $Depth)"
