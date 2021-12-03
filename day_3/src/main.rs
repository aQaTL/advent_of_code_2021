extern crate itertools;

use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_3/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u32> {
	let n_len = input
		.lines()
		.next()
		.map(str::len)
		.ok_or_else(|| anyhow!("expected a line"))?;
	let input = input.lines().map(str::as_bytes).collect_vec();
	let (mut gamma, mut epsilon) = (0, 0);

	for n in 0..n_len {
		let (ones_count, zeros_count) = count_ones_and_zeros(&input, n);

		gamma <<= 1;
		epsilon <<= 1;
		if ones_count > zeros_count {
			gamma |= 1;
		} else {
			epsilon |= 1;
		}
	}

	Ok(gamma * epsilon)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let n_len = input
		.lines()
		.next()
		.map(|x| x.len())
		.ok_or_else(|| anyhow!("expected a line"))?;
	let mut input = input.lines().collect_vec();
	let mut input2 = input.clone();

	for n in 0..n_len {
		if input.len() > 1 {
			let (ones_count, zeros_count) = count_ones_and_zeros(&input, n);
			let retained_byte = if ones_count >= zeros_count {
				b'1'
			} else {
				b'0'
			};
			input.retain(|line| line.as_bytes()[n] == retained_byte);
		}

		if input2.len() > 1 {
			let (ones_count_2, zeros_count_2) = count_ones_and_zeros(&input2, n);
			let retained_byte = if ones_count_2 >= zeros_count_2 {
				b'0'
			} else {
				b'1'
			};
			input2.retain(|line| line.as_bytes()[n] == retained_byte);
		}
	}

	let oxygen_generator_rating = u64::from_str_radix(input[0], 2)?;
	let co2_scrubber_rating = u64::from_str_radix(input2[0], 2)?;

	Ok(oxygen_generator_rating * co2_scrubber_rating)
}

fn count_ones_and_zeros<T: AsRef<[u8]>>(input: &[T], idx: usize) -> (u64, u64) {
	input.iter().fold((0, 0), |(mut ones, mut zeros), line| {
		match line.as_ref()[idx] {
			b'0' => zeros += 1,
			b'1' => ones += 1,
			e => panic!("unexpected byte: {}", e),
		}
		(ones, zeros)
	})
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

	#[test]
	fn part_1() -> anyhow::Result<()> {
		assert_eq!(super::part_1(EX_1)?, 198);
		Ok(())
	}

	#[test]
	fn part_2() -> anyhow::Result<()> {
		assert_eq!(super::part_2(EX_1)?, 230);
		Ok(())
	}
}
