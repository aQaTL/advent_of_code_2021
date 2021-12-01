use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input_numbers: Vec<_> = std::fs::read_to_string("day_1/input.txt")?
		.lines()
		.map(|x| x.parse::<u64>().unwrap())
		.collect();

	let mut count = 0;
	for (a, b) in input_numbers.iter().tuple_windows() {
		if b > a {
			count += 1;
		}
	}
	println!("Part 1: {}", count);

	let (mut count, mut previous) = (0, u64::MAX);
	for (a, b, c) in input_numbers.iter().tuple_windows() {
		let sum = a + b + c;
		if sum > previous {
			count += 1;
		}
		previous = sum;
	}
	println!("Part 2: {}", count);

	Ok(())
}
