extern crate itertools;
use anyhow::anyhow;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_12/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

#[derive(Clone)]
struct Path<'a> {
	explored: HashSet<&'a str>,
	key: &'a str,
}

fn part_1<'a>(input: &'a str) -> anyhow::Result<i64> {
	let mut graph = HashMap::<&'a str, Vec<&'a str>>::new();
	for line in input.lines() {
		let (key, val) = line
			.split('-')
			.next_tuple()
			.ok_or_else(|| anyhow!("malformed input"))?;
		graph.entry(key).or_default().push(val);
		graph.entry(val).or_default().push(key);
	}

	let mut queue: VecDeque<Path<'a>> = VecDeque::from([Path {
		explored: HashSet::from(["start"]),
		key: "start",
	}]);

	let mut paths = 0;

	while let Some(path) = queue.pop_front() {
		if path.key == "end" {
			paths += 1;
			continue;
		}

		if let Some(vals) = graph.get(path.key) {
			'vals_loop: for val in vals {
				let mut new_path = path.clone();
				if is_ascii_lowercase(*val) {
					if path.explored.contains(val) {
						continue 'vals_loop;
					}
					new_path.explored.insert(*val);
				}
				new_path.key = *val;
				queue.push_back(new_path);
			}
		}
	}

	Ok(paths)
}

#[derive(Clone)]
struct PathP2<'a> {
	explored: HashMap<&'a str, u8>,
	key: &'a str,
}

fn part_2<'a>(input: &'a str) -> anyhow::Result<i64> {
	let mut graph = HashMap::<&'a str, Vec<&'a str>>::new();
	for line in input.lines() {
		let (key, val) = line
			.split('-')
			.next_tuple()
			.ok_or_else(|| anyhow!("malformed input"))?;
		graph.entry(key).or_default().push(val);
		graph.entry(val).or_default().push(key);
	}

	let mut queue: VecDeque<PathP2<'a>> = VecDeque::from([PathP2 {
		explored: HashMap::from([("start", 1)]),
		key: "start",
	}]);

	let mut paths = 0;

	while let Some(path) = queue.pop_front() {
		if path.key == "end" {
			paths += 1;
			continue;
		}

		if let Some(vals) = graph.get(path.key) {
			'vals_loop: for val in vals {
				if *val == "start" {
					continue 'vals_loop;
				}
				let mut new_path = path.clone();
				if is_ascii_lowercase(*val) {
					if let Some(count) = path.explored.get(val) {
						if *count >= 2 {
							continue 'vals_loop;
						}
					}
					*new_path.explored.entry(*val).or_default() += 1;
					if new_path.explored.values().filter(|val| **val >= 2).count() >= 2 {
						continue 'vals_loop;
					}
				}
				new_path.key = *val;
				queue.push_back(new_path);
			}
		}
	}

	Ok(paths)
}

fn is_ascii_lowercase(s: &str) -> bool {
	s.as_bytes().iter().all(|b| b.is_ascii_lowercase())
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
	static EX_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

	static EX_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

	#[test]
	fn part_1_ex_1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 10);
	}

	#[test]
	fn part_1_ex_2() {
		assert_eq!(super::part_1(EX_2).unwrap(), 19);
	}

	#[test]
	fn part_1_ex_3() {
		assert_eq!(super::part_1(EX_3).unwrap(), 226);
	}

	#[test]
	fn part_2_ex_1() {
		assert_eq!(super::part_2(EX_1).unwrap(), 36);
	}

	#[test]
	fn part_2_ex_2() {
		assert_eq!(super::part_2(EX_2).unwrap(), 103);
	}

	#[test]
	fn part_2_ex_3() {
		assert_eq!(super::part_2(EX_3).unwrap(), 3509);
	}
}
