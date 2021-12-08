extern crate itertools;

use anyhow::{anyhow, bail};
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_8/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let mut count = 0;
	for line in input.lines() {
		let (_unique_patterns, output_values) = line
			.split('|')
			.next_tuple()
			.ok_or_else(|| anyhow!("malformed input"))?;

		for output in output_values.trim().split(' ') {
			match output.len() {
				2 | 4 | 3 | 7 => count += 1,
				_ => (),
			}
		}
	}

	Ok(count)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let mut count = 0;
	for line in input.lines() {
		let (unique_patterns, output_values) = line
			.split('|')
			.next_tuple()
			.ok_or_else(|| anyhow!("malformed input"))?;
		let unique_patterns: Vec<&[u8]> = unique_patterns
			.trim()
			.split(' ')
			.map(|x| x.as_bytes())
			.collect();
		let output_values: Vec<&[u8]> = output_values
			.trim()
			.split(' ')
			.map(|x| x.as_bytes())
			.collect();

		let mut one = "".as_bytes();
		let mut seven = "".as_bytes();
		let mut four = "".as_bytes();
		let mut eight = "".as_bytes();
		for pat in &unique_patterns {
			match pat.len() {
				2 => one = pat,
				3 => seven = pat,
				4 => four = pat,
				7 => eight = pat,
				_ => (),
			}
		}

		let mut top_right = 0;
		let mut six = "".as_bytes();

		let six_lens = unique_patterns
			.iter()
			.filter(|x| x.len() == 6)
			.collect_vec();
		for six_len in &six_lens {
			if !six_len.contains(&one[0]) {
				six = **six_len;
				top_right = one[0];
			}
			if !six_len.contains(&one[1]) {
				six = **six_len;
				top_right = one[1];
			}
		}

		let bottom_right = one
			.iter()
			.find(|x| **x != top_right)
			.ok_or_else(|| anyhow!("failed to find bottom right segment"))?;

		let mut three = "".as_bytes();
		let mut two = "".as_bytes();
		let mut five = "".as_bytes();

		for five_len in unique_patterns.iter().filter(|x| x.len() == 5) {
			if five_len.contains(&one[0]) && five_len.contains(&one[1]) {
				three = *five_len;
			}
			if !five_len.contains(&top_right) {
				five = *five_len;
			}
			if !five_len.contains(bottom_right) {
				two = *five_len;
			}
		}

		let mut nine = five.to_owned();
		nine.push(top_right);
		nine.sort_unstable();

		let one: Vec<u8> = one.iter().cloned().sorted().collect_vec();
		let two: Vec<u8> = two.iter().cloned().sorted().collect_vec();
		let three: Vec<u8> = three.iter().cloned().sorted().collect_vec();
		let four: Vec<u8> = four.iter().cloned().sorted().collect_vec();
		let five: Vec<u8> = five.iter().cloned().sorted().collect_vec();
		let six: Vec<u8> = six.iter().cloned().sorted().collect_vec();
		let seven: Vec<u8> = seven.iter().cloned().sorted().collect_vec();
		let eight: Vec<u8> = eight.iter().cloned().sorted().collect_vec();

		let zero: Vec<u8> = six_lens
			.iter()
			.map(|six_len| six_len.iter().cloned().sorted().collect_vec())
			.find(|six_len| *six_len != six && *six_len != nine)
			.ok_or_else(|| anyhow!("failed to find zero"))?;

		let mut out = String::new();
		for output_value in &output_values {
			let output_value = output_value.iter().cloned().sorted().collect_vec();

			if output_value == zero {
				out.push('0');
			} else if output_value == one {
				out.push('1');
			} else if output_value == two {
				out.push('2');
			} else if output_value == three {
				out.push('3');
			} else if output_value == four {
				out.push('4');
			} else if output_value == five {
				out.push('5');
			} else if output_value == six {
				out.push('6');
			} else if output_value == seven {
				out.push('7');
			} else if output_value == eight {
				out.push('8');
			} else if output_value == nine {
				out.push('9');
			} else {
				bail!(
					"failed to match on output value: {}",
					std::str::from_utf8(&output_value)?
				);
			}
		}
		count += out.parse::<i64>()?;
	}

	Ok(count)
}

#[cfg(test)]
mod tests {
	static EX_1: &str =
		"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

	static EX_2: &str =
		"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

	#[test]
	fn part_1_ex1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 0);
	}

	#[test]
	fn part_1_ex2() {
		assert_eq!(super::part_1(EX_2).unwrap(), 26);
	}

	#[test]
	fn part_2_ex1() {
		assert_eq!(super::part_2(EX_1).unwrap(), 5353);
	}

	#[test]
	fn part_2_ex2() {
		assert_eq!(super::part_2(EX_2).unwrap(), 61229);
	}
}
