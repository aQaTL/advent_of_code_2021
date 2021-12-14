extern crate itertools;
extern crate nom;

use anyhow::anyhow;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, anychar, line_ending, multispace0};
use nom::combinator::map;
use nom::multi::fold_many1;
use nom::sequence::{separated_pair, terminated, tuple};
use nom::IResult;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_14/input.txt")?;
	println!("Part 1: {}", solve(&input, 10)?);
	println!("Part 1: {}", solve(&input, 40)?);
	Ok(())
}

fn solve(input: &str, iterations: usize) -> anyhow::Result<u64> {
	let (
		_,
		Input {
			template,
			pair_insertions,
		},
	) = parse_input(input).map_err(|err| anyhow!("{:?}", err))?;

	let mut hm = HashMap::<(u8, u8), u64>::new();
	for (a, b) in template.as_bytes().iter().tuple_windows() {
		*hm.entry((*a, *b)).or_default() += 1;
	}

	for _ in 0..iterations {
		let mut hm_clone = hm.clone();
		for (pair, insertion) in pair_insertions.iter() {
			if let Some(v) = hm.get_mut(pair) {
				*hm_clone.entry((pair.0, *insertion)).or_default() += *v;
				*hm_clone.entry((*insertion, pair.1)).or_default() += *v;
				*hm_clone.entry(*pair).or_default() -= *v;
			}
		}
		hm = hm_clone;
	}

	let mut hm_count = HashMap::<u8, u64>::new();
	for ((_, b), v) in &hm {
		*hm_count.entry(*b).or_default() += *v;
	}
	let max = hm_count.iter().map(|(_, v)| *v).max().unwrap();
	let min = hm_count.iter().map(|(_, v)| *v).min().unwrap();

	Ok(max - min)
}

struct Input<'a> {
	template: &'a str,
	pair_insertions: HashMap<(u8, u8), u8>,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
	let (input, template) = alpha1(input)?;
	let (input, _) = multispace0(input)?;
	let (input, pair_insertions) = fold_many1(
		terminated(
			separated_pair(
				map(tuple((anychar, anychar)), |(a, b)| (a as u8, b as u8)),
				tag(" -> "),
				map(anychar, |c| c as u8),
			),
			line_ending,
		),
		HashMap::new,
		|mut hm, (key, val)| {
			hm.insert(key, val);
			hm
		},
	)(input)?;
	Ok((
		input,
		Input {
			template,
			pair_insertions,
		},
	))
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

	#[test]
	fn part_1() {
		assert_eq!(super::solve(EX_1, 10).unwrap(), 1588);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::solve(EX_1, 40).unwrap(), 2188189693529);
	}
}
