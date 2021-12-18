extern crate itertools;
extern crate nom;

use anyhow::anyhow;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::char;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_18/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
	x: V,
	y: V,
	depth: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum V {
	Pair(Box<Pair>),
	Value(i32),
}

fn part_1(input: &str) -> anyhow::Result<i32> {
	Ok(magnitude(&snailfish_add(
		input
			.lines()
			.map(parse_pair(0))
			.map(|res| res.map(|(_, pair)| pair))
			.try_collect()
			.map_err(|err| err.to_owned())?,
	)))
}

fn part_2(input: &str) -> anyhow::Result<i32> {
	let pairs: Vec<Pair> = input
		.lines()
		.map(parse_pair(0))
		.map(|res| res.map(|(_, pair)| pair))
		.try_collect()
		.map_err(|err| err.to_owned())?;

	let max_magnitude = pairs
		.iter()
		.permutations(2)
		.map(|two_numbers| {
			magnitude(&snailfish_add(
				two_numbers.into_iter().cloned().collect_vec(),
			))
		})
		.max()
		.ok_or_else(|| anyhow!("wtf"))?;

	Ok(max_magnitude)
}

fn magnitude(p: &Pair) -> i32 {
	match (&p.x, &p.y) {
		(V::Value(x), V::Value(y)) => (3 * (*x)) + (2 * (*y)),
		(V::Pair(inner_pair), V::Value(y)) => (3 * magnitude(inner_pair)) + (2 * (*y)),
		(V::Value(x), V::Pair(inner_pair)) => (3 * (*x)) + (2 * magnitude(inner_pair)),
		(V::Pair(inner_pair_x), V::Pair(inner_pair_y)) => {
			(3 * magnitude(inner_pair_x)) + (2 * magnitude(inner_pair_y))
		}
	}
}

fn snailfish_add(mut pairs: Vec<Pair>) -> Pair {
	let mut pair: Pair = pairs.remove(0);

	loop {
		loop {
			match try_explode(&mut pair) {
				ExplodeAction::Exploded
				| ExplodeAction::XExploded { .. }
				| ExplodeAction::YExploded { .. } => continue,
				ExplodeAction::Nothing => (),
			}
			if !try_split(&mut pair) {
				break;
			}
		}
		if pairs.is_empty() {
			break;
		}
		let old_res = pair;
		pair = Pair {
			x: V::Pair(Box::new(old_res)),
			y: V::Pair(Box::new(pairs.remove(0))),
			depth: -1,
		};
		increment_depths(&mut pair);
	}

	pair
}

fn increment_depths(p: &mut Pair) {
	p.depth += 1;
	if let V::Pair(pair) = &mut p.x {
		increment_depths(pair.as_mut());
	}
	if let V::Pair(pair) = &mut p.y {
		increment_depths(pair.as_mut());
	}
}

#[derive(Debug)]
enum ExplodeAction {
	Nothing,
	YExploded { y: i32 },
	XExploded { x: i32 },
	Exploded,
}

fn try_explode(p: &mut Pair) -> ExplodeAction {
	if let V::Pair(inner_p) = &mut p.x {
		if inner_p.depth == 4 {
			let (px, py) = match (&inner_p.x, &inner_p.y) {
				(V::Value(x), V::Value(y)) => (*x, *y),
				_ => unreachable!(),
			};

			p.x = V::Value(0);
			match p.y {
				V::Value(ref mut v) => *v += py,
				V::Pair(ref mut y_pair) => match y_pair.x {
					V::Value(ref mut v) => {
						*v += py;
					}
					_ => unreachable!(),
				},
			}
			return ExplodeAction::XExploded { x: px };
		}
		match try_explode(inner_p.as_mut()) {
			ExplodeAction::Nothing => (),
			ExplodeAction::YExploded { y } => {
				match &mut p.y {
					V::Pair(p) => add_x(p.as_mut(), y),
					V::Value(ref mut v) => *v += y,
				}
				return ExplodeAction::Exploded;
			}
			ExplodeAction::XExploded { x } => return ExplodeAction::XExploded { x },
			ExplodeAction::Exploded => return ExplodeAction::Exploded,
		}
	}
	if let V::Pair(inner_p) = &mut p.y {
		if inner_p.depth == 4 {
			let (px, py) = match (&inner_p.x, &inner_p.y) {
				(V::Value(x), V::Value(y)) => (*x, *y),
				_ => unreachable!(),
			};

			p.y = V::Value(0);
			match p.x {
				V::Value(ref mut v) => *v += px,
				_ => unreachable!(),
			}
			return ExplodeAction::YExploded { y: py };
		}
		match try_explode(inner_p.as_mut()) {
			ExplodeAction::Nothing => (),
			ExplodeAction::YExploded { y } => return ExplodeAction::YExploded { y },
			ExplodeAction::XExploded { x } => {
				match &mut p.x {
					V::Pair(p) => add_y(p.as_mut(), x),
					V::Value(ref mut v) => *v += x,
				}
				return ExplodeAction::Exploded;
			}
			ExplodeAction::Exploded => return ExplodeAction::Exploded,
		}
	}
	ExplodeAction::Nothing
}

fn add_x(p: &mut Pair, x: i32) {
	match &mut p.x {
		V::Value(ref mut v) => *v += x,
		V::Pair(p) => add_x(p.as_mut(), x),
	}
}

fn add_y(p: &mut Pair, y: i32) {
	match &mut p.y {
		V::Value(ref mut v) => *v += y,
		V::Pair(p) => add_y(p.as_mut(), y),
	}
}

fn try_split(p: &mut Pair) -> bool {
	match &mut p.x {
		V::Value(x) => {
			if *x >= 10 {
				p.x = V::Pair(Box::new(Pair {
					x: V::Value(*x / 2),
					y: V::Value(if *x % 2 == 0 { *x / 2 } else { (*x / 2) + 1 }),
					depth: p.depth + 1,
				}));
				return true;
			}
		}
		V::Pair(inner_pair) => {
			if try_split(inner_pair.as_mut()) {
				return true;
			}
		}
	}
	match &mut p.y {
		V::Value(y) => {
			if *y >= 10 {
				p.y = V::Pair(Box::new(Pair {
					x: V::Value(*y / 2),
					y: V::Value(if *y % 2 == 0 { *y / 2 } else { (*y / 2) + 1 }),
					depth: p.depth + 1,
				}));
				true
			} else {
				false
			}
		}
		V::Pair(inner_pair) => try_split(inner_pair.as_mut()),
	}
}

fn parse_pair(current_depth: i32) -> impl FnMut(&str) -> IResult<&str, Pair> {
	move |input| {
		map(
			delimited(
				char('['),
				separated_pair(parse_v(current_depth), char(','), parse_v(current_depth)),
				char(']'),
			),
			|(x, y)| Pair {
				x,
				y,
				depth: current_depth,
			},
		)(input)
	}
}

fn parse_v(current_depth: i32) -> impl FnMut(&str) -> IResult<&str, V> {
	move |input| {
		alt((
			map(i32, V::Value),
			map(parse_pair(current_depth + 1), |pair| {
				V::Pair(Box::new(pair))
			}),
		))(input)
	}
}

#[cfg(test)]
mod tests {
	use super::{ExplodeAction, Pair};
	use itertools::Itertools;

	static EX_1: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

	static EX_2: &str = "[1,2]
[[3,4],5]
";

	static EX_3: &str = "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]
";

	static EX_4: &str = "[1,1]
[2,2]
[3,3]
[4,4]
";

	static EX_5: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
";

	static EX_6: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";

	static EX_7: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

	#[test]
	fn p1_ex1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 4140);
	}

	#[test]
	fn p1_ex2() {
		assert_eq!(super::part_1(EX_2).unwrap(), 143);
	}

	#[test]
	fn p1_ex3() {
		assert_eq!(super::part_1(EX_3).unwrap(), 1384);
	}

	#[test]
	fn p1_ex3_snailfish_add() {
		let pairs: Vec<Pair> = EX_3
			.lines()
			.map(super::parse_pair(0))
			.map(|res| res.map(|(_, pair)| pair))
			.try_collect()
			.unwrap();
		let pair = super::snailfish_add(pairs);
		let (_, expected) = super::parse_pair(0)("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_ex4() {
		assert_eq!(super::part_1(EX_4).unwrap(), 445);
	}

	#[test]
	fn p1_ex5() {
		assert_eq!(super::part_1(EX_5).unwrap(), 791);
	}

	#[test]
	fn p1_ex6() {
		assert_eq!(super::part_1(EX_6).unwrap(), 1137);
	}

	#[test]
	fn p1_ex7() {
		assert_eq!(super::part_1(EX_7).unwrap(), 3488);
	}

	#[test]
	fn p1_snailfish_add_1() {
		let pairs: Vec<Pair> = EX_7
			.lines()
			.map(super::parse_pair(0))
			.map(|res| res.map(|(_, pair)| pair))
			.try_collect()
			.unwrap();
		let pair = super::snailfish_add(pairs);
		let (_, expected) =
			super::parse_pair(0)("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_snailfish_add_2() {
		let pairs: Vec<Pair> = EX_7
			.lines()
			.take(2)
			.map(super::parse_pair(0))
			.map(|res| res.map(|(_, pair)| pair))
			.try_collect()
			.unwrap();
		let pair = super::snailfish_add(pairs);
		let (_, expected) =
			super::parse_pair(0)("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
				.unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_exploding_1() {
		let (_, mut pair) = super::parse_pair(0)("[[[[[9,8],1],2],3],4]").unwrap();
		let explode_action = super::try_explode(&mut pair);
		assert!(matches!(explode_action, ExplodeAction::XExploded { .. }));
		let (_, expected) = super::parse_pair(0)("[[[[0,9],2],3],4]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_exploding_2() {
		let (_, mut pair) = super::parse_pair(0)("[7,[6,[5,[4,[3,2]]]]]").unwrap();
		let explode_action = super::try_explode(&mut pair);
		assert!(matches!(explode_action, ExplodeAction::YExploded { .. }));
		let (_, expected) = super::parse_pair(0)("[7,[6,[5,[7,0]]]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_exploding_3() {
		let (_, mut pair) = super::parse_pair(0)("[[6,[5,[4,[3,2]]]],1]").unwrap();
		let explode_action = super::try_explode(&mut pair);
		assert!(matches!(explode_action, ExplodeAction::Exploded));
		let (_, expected) = super::parse_pair(0)("[[6,[5,[7,0]]],3]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_exploding_4() {
		let (_, mut pair) = super::parse_pair(0)("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
		let explode_action = super::try_explode(&mut pair);
		assert!(matches!(explode_action, ExplodeAction::Exploded));
		let (_, expected) = super::parse_pair(0)("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_exploding_5() {
		let (_, mut pair) = super::parse_pair(0)("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
		let explode_action = super::try_explode(&mut pair);
		assert!(matches!(explode_action, ExplodeAction::YExploded { .. }));
		let (_, expected) = super::parse_pair(0)("[[3,[2,[8,0]]],[9,[5,[7,0]]]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_splitting_1() {
		let (_, mut pair) = super::parse_pair(0)("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
		let did_split = super::try_split(&mut pair);
		assert!(did_split);
		let (_, expected) = super::parse_pair(0)("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p1_splitting_2() {
		let (_, mut pair) = super::parse_pair(0)("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();
		let did_split = super::try_split(&mut pair);
		assert!(did_split);
		let (_, expected) = super::parse_pair(0)("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]").unwrap();
		assert_eq!(pair, expected);
	}

	#[test]
	fn p2_ex1() {
		assert_eq!(super::part_2(EX_1).unwrap(), 3993);
	}
}
