use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_5/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<usize> {
	let mut hm = HashMap::<(i32, i32), u32>::new();
	for line in input.lines() {
		let (_, ((x1, y1), (x2, y2))) = parse_line(line).map_err(|e| anyhow::anyhow!("{:?}", e))?;

		if x1 == x2 || y1 == y2 {
			for j in y1.min(y2)..=y1.max(y2) {
				for i in x1.min(x2)..=x1.max(x2) {
					*hm.entry((i, j)).or_default() += 1;
				}
			}
		}
	}

	Ok(hm.values().filter(|v| **v > 1).count())
}

fn part_2(input: &str) -> anyhow::Result<usize> {
	let mut hm = HashMap::<(i32, i32), u32>::new();
	for line in input.lines() {
		let (_, ((x1, y1), (x2, y2))) = parse_line(line).map_err(|e| anyhow::anyhow!("{:?}", e))?;

		if x1 == x2 || y1 == y2 {
			for j in y1.min(y2)..=y1.max(y2) {
				for i in x1.min(x2)..=x1.max(x2) {
					*hm.entry((i, j)).or_default() += 1;
				}
			}
		} else {
			if x1 <= x2 && y1 <= y2 {
				for (i, j) in (x1..=x2).zip(y1..=y2) {
					*hm.entry((i, j)).or_default() += 1;
				}
			} else if x1 > x2 && y1 <= y2 {
				for (i, j) in ((x2..=x1).rev()).zip(y1..=y2) {
					*hm.entry((i, j)).or_default() += 1;
				}
			} else if x1 <= x2 && y1 > y2 {
				for (i, j) in (x1..=x2).zip((y2..=y1).rev()) {
					*hm.entry((i, j)).or_default() += 1;
				}
			} else if x1 > x2 && y1 > y2 {
				for (i, j) in ((x2..=x1).rev()).zip((y2..=y1).rev()) {
					*hm.entry((i, j)).or_default() += 1;
				}
			}
		}
	}

	Ok(hm.values().filter(|v| **v > 1).count())
}

fn parse_line(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
	use nom::character::complete::i32;
	separated_pair(
		separated_pair(i32, char(','), i32),
		tag(" -> "),
		separated_pair(i32, char(','), i32),
	)(input)
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

	#[test]
	fn part_1() -> anyhow::Result<()> {
		assert_eq!(super::part_1(EX_1)?, 5);
		Ok(())
	}

	#[test]
	fn part_2() -> anyhow::Result<()> {
		assert_eq!(super::part_2(EX_1)?, 12);
		Ok(())
	}
}
