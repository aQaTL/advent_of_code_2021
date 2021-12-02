use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_2/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let (mut depth, mut horizontal) = (0, 0);

	for (cmd, n) in input.lines().filter_map(|l| l.split(" ").next_tuple()) {
		let n = n.parse::<i64>()?;
		match cmd {
			"forward" => horizontal += n,
			"down" => depth += n,
			"up" => depth -= n,
			cmd => bail!("unknown cmd: {}", cmd),
		}
	}

	Ok(horizontal * depth)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let (mut depth, mut horizontal, mut aim) = (0, 0, 0);

	for (cmd, n) in input.lines().filter_map(|l| l.split(" ").next_tuple()) {
		let n = n.parse::<i64>()?;
		match cmd {
			"forward" => {
				horizontal += n;
				depth += aim * n
			}
			"down" => aim += n,
			"up" => aim -= n,
			cmd => bail!("unknown cmd: {}", cmd),
		}
	}

	Ok(horizontal * depth)
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

	#[test]
	fn p1_test() -> anyhow::Result<()> {
		assert_eq!(super::part_1(EX_1)?, 150);
		Ok(())
	}

	#[test]
	fn p2_test() -> anyhow::Result<()> {
		assert_eq!(super::part_2(EX_1)?, 900);
		Ok(())
	}
}
