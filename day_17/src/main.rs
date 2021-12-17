use anyhow::anyhow;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_17/input.txt")?;
	let TrickShot {
		highest_init_y_velocity,
		possible_init_velocities,
	} = trick_shot(&input)?;
	println!("Part 1: {}", highest_init_y_velocity);
	println!("Part 2: {}", possible_init_velocities);
	Ok(())
}

struct TrickShot {
	highest_init_y_velocity: i32,
	possible_init_velocities: usize,
}

fn trick_shot(input: &str) -> anyhow::Result<TrickShot> {
	let (_, (x_range, y_range)) = parse_input(input).map_err(|err| anyhow!("{:?}", err))?;

	let mut highest_init_y_velocity = 0;
	let mut hs = HashSet::new();

	for init_x in 0..=*x_range.end() {
		for init_y in *y_range.start()..=1000 {
			let (mut x, mut y) = (0, 0);
			let (mut x_velocity, mut y_velocity) = (init_x, init_y);

			let mut highest_y = 0;
			while y >= *y_range.start() {
				x += x_velocity;
				y += y_velocity;
				highest_y = highest_y.max(y);
				if x_velocity > 0 {
					x_velocity -= 1;
				} else if x_velocity < 0 {
					x_velocity += 1;
				}
				y_velocity -= 1;
				if x_range.contains(&x) && y_range.contains(&y) {
					highest_init_y_velocity = highest_init_y_velocity.max(highest_y);
					hs.insert((init_x, init_y));
				}
			}
		}
	}

	Ok(TrickShot {
		highest_init_y_velocity,
		possible_init_velocities: hs.len(),
	})
}

fn parse_input(input: &str) -> IResult<&str, (RangeInclusive<i32>, RangeInclusive<i32>)> {
	let (input, _) = tag("target area: ")(input)?;
	let (input, x_range) = preceded(
		tag("x="),
		map(separated_pair(i32, tag(".."), i32), |(min, max)| min..=max),
	)(input)?;
	let (input, _) = tag(", ")(input)?;
	let (input, y_range) = preceded(
		tag("y="),
		map(separated_pair(i32, tag(".."), i32), |(min, max)| min..=max),
	)(input)?;
	Ok((input, (x_range, y_range)))
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "target area: x=20..30, y=-10..-5";

	#[test]
	fn part_1() {
		assert_eq!(super::trick_shot(EX_1).unwrap().highest_init_y_velocity, 45);
	}

	#[test]
	fn part_2() {
		assert_eq!(
			super::trick_shot(EX_1).unwrap().possible_init_velocities,
			112
		);
	}
}
