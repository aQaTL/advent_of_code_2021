extern crate nom;

use anyhow::anyhow;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i32, line_ending, multispace0};
use nom::combinator::map;
use nom::multi::{fold_many1, many1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_13/input.txt")?;
	let Answer { part_1, part_2 } = solve(&input)?;
	println!("Part 1: {}", part_1);
	println!("Part 2:\n{}", part_2);
	Ok(())
}

struct Answer {
	part_1: usize,
	part_2: String,
}

fn solve(input: &str) -> anyhow::Result<Answer> {
	let (
		_,
		Input {
			mut paper,
			fold_along,
		},
	) = parse(input).map_err(|err| anyhow!("{:?}", err))?;
	let mut width = paper
		.iter()
		.max_by_key(|(x, _)| x)
		.ok_or_else(|| anyhow!("wtf"))?
		.0 + 1;
	let mut height = paper
		.iter()
		.max_by_key(|(_, y)| y)
		.ok_or_else(|| anyhow!("wtf"))?
		.1 + 1;

	let mut part_1 = 0;
	for (idx, fold) in fold_along.into_iter().enumerate() {
		match fold {
			FoldAlong::X(x) => {
				let new_width = x;
				for x in (new_width + 1)..width {
					for y in 0..height {
						if paper.remove(&(x, y)) {
							paper.insert((new_width - 1 - (x - new_width - 1), y));
						}
					}
				}
				width = new_width;
			}
			FoldAlong::Y(y) => {
				let new_height = y;
				for x in 0..width {
					for y in (new_height + 1)..height {
						if paper.remove(&(x, y)) {
							paper.insert((x, new_height - 1 - (y - new_height - 1)));
						}
					}
				}
				height = new_height;
			}
		}
		if idx == 0 {
			part_1 = paper.len();
		}
	}
	let mut part_2 = String::new();
	for y in 0..height {
		for x in 0..width {
			match paper.get(&(x, y)) {
				Some(_) => part_2.push('\u{2588}'),
				None => part_2.push(' '),
			}
		}
		part_2.push('\n');
	}

	Ok(Answer { part_1, part_2 })
}

fn parse(input: &str) -> IResult<&str, Input> {
	let (input, paper) = fold_many1(
		terminated(
			map(separated_pair(i32, char(','), i32), |(x, y)| {
				(x as u32, y as u32)
			}),
			line_ending,
		),
		HashSet::new,
		|mut acc: HashSet<(u32, u32)>, key| {
			acc.insert(key);
			acc
		},
	)(input)?;

	let (input, _) = multispace0(input)?;

	let (input, fold_along) = many1(terminated(
		preceded(
			tag("fold along "),
			alt((
				map(preceded(tag("x="), i32), |x| FoldAlong::X(x as u32)),
				map(preceded(tag("y="), i32), |y| FoldAlong::Y(y as u32)),
			)),
		),
		line_ending,
	))(input)?;

	Ok((input, Input { paper, fold_along }))
}

struct Input {
	paper: HashSet<(u32, u32)>,
	fold_along: Vec<FoldAlong>,
}

#[derive(Debug)]
enum FoldAlong {
	X(u32),
	Y(u32),
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

	#[test]
	fn part_1() {
		assert_eq!(super::solve(EX_1).unwrap().part_1, 17);
	}
}
