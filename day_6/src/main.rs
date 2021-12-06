fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_6/input.txt")?;
	println!("Part 1: {}", grow_fishes(&input, 80)?);
	println!("Part 2: {}", grow_fishes(&input, 256)?);
	Ok(())
}

fn grow_fishes(input: &str, iterations: u64) -> anyhow::Result<u64> {
	let mut timers: Vec<u64> = vec![0_u64; 9];
	for x in input.trim().split(",").map(|x| x.parse::<usize>()) {
		timers[x?] += 1;
	}
	for _ in 0..iterations {
		timers[0..=8].rotate_left(1);
		timers[6] += timers[8];
	}
	Ok(timers.iter().sum())
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "3,4,3,1,2";

	#[test]
	fn part_1_ex_1() {
		assert_eq!(super::grow_fishes(EX_1, 18).unwrap(), 26);
	}

	#[test]
	fn part_1() {
		assert_eq!(super::grow_fishes(EX_1, 80).unwrap(), 5934);
	}

	#[test]
	fn part_1_single_fish() {
		assert_eq!(super::grow_fishes("3", 5).unwrap(), 2);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::grow_fishes(EX_1, 256).unwrap(), 26984457539);
	}
}
