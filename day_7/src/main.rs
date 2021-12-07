use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_7/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let mut nums: Vec<_> = input
		.trim()
		.split(",")
		.map(|x| x.parse::<i64>())
		.try_collect()?;
	nums.sort();

	let median = if nums.len() % 2 == 0 {
		nums[nums.len() / 2]
	} else {
		(nums[nums.len() / 2] + nums[(nums.len() / 2) + 1]) / 2
	};

	Ok(nums.iter().map(|x| (x - median).abs()).sum())
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let nums: Vec<_> = input
		.trim()
		.split(",")
		.map(|x| x.parse::<i64>())
		.try_collect()?;

	let average = nums.iter().sum::<i64>() as f64 / (nums.len() as f64);
	let average_floor = average.floor() as i64;
	let average_ceil = average.ceil() as i64;

	let with_floored_average: i64 = nums
		.iter()
		.map(|x| (1..=(x - average_floor).abs()).sum::<i64>())
		.sum();
	let with_ceiled_average: i64 = nums
		.iter()
		.map(|x| (1..=(x - average_ceil).abs()).sum::<i64>())
		.sum();

	Ok(with_floored_average.min(with_ceiled_average))
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "16,1,2,0,4,2,7,1,2,14";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 37);
	}
	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EX_1).unwrap(), 168);
	}
}
