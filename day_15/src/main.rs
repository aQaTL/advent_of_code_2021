extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_15/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<i64> {
	let input: Vec<Vec<i64>> = input
		.lines()
		.map(|line| line.bytes().map(|x| (x - b'0') as i64).collect_vec())
		.collect_vec();
	let mut costs = HashMap::<(usize, usize), i64>::new();
	let mut parents = HashMap::<(usize, usize), (usize, usize)>::new();
	let mut processed = HashSet::<(usize, usize)>::new();

	parents.insert((0, 1), (0, 0));
	parents.insert((1, 0), (0, 0));

	costs.insert((0, 1), input[1][0]);
	costs.insert((1, 0), input[0][1]);
	let (last_i, last_j) = (input.last().unwrap().len() - 1, input.len() - 1);
	costs.insert((last_i, last_j), i64::MAX);

	while let Some(node @ (i, j)) = find_lowest_cost_node(&costs, &processed) {
		let cost = *costs.get(&node).unwrap();
		let neighbours = deltas((i, j), input[0].len(), input.len());
		for (ni, nj) in neighbours {
			let new_cost = cost + input[nj][ni];
			if *costs.get(&(ni, nj)).unwrap_or(&i64::MAX) > new_cost {
				costs.insert((ni, nj), new_cost);
				*parents.entry((ni, nj)).or_default() = node;
			}
		}
		processed.insert(node);
	}

	Ok(*costs.get(&(last_i, last_j)).unwrap())
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let mut input: Vec<Vec<i64>> = input
		.lines()
		.map(|line| line.bytes().map(|x| (x - b'0') as i64).collect_vec())
		.collect_vec();
	let size = input.len();

	for c in 1..5 {
		for j in 0..size {
			let mut vec = Vec::with_capacity(size * 5);
			let next = &input[(c - 1) * size + j];
			for val in next {
				let mut next_val = *val + 1;
				if next_val > 9 {
					next_val -= 9;
				}
				vec.push(next_val);
			}
			input.push(vec);
		}
	}

	for j in 0..(size * 5) {
		for c in 1..5 {
			for i in 0..size {
				let mut next = input[j][(c - 1) * size + i] + 1;
				if next > 9 {
					next -= 9;
				}
				input[j].push(next);
			}
		}
	}

	let mut costs = HashMap::<(usize, usize), i64>::new();
	let mut parents = HashMap::<(usize, usize), (usize, usize)>::new();
	let mut processed = HashSet::<(usize, usize)>::new();

	parents.insert((0, 1), (0, 0));
	parents.insert((1, 0), (0, 0));

	costs.insert((0, 1), input[1][0]);
	costs.insert((1, 0), input[0][1]);
	let (last_i, last_j) = (input.last().unwrap().len() - 1, input.len() - 1);
	costs.insert((last_i, last_j), i64::MAX);

	while let Some(node @ (i, j)) = find_lowest_cost_node(&costs, &processed) {
		let cost = *costs.get(&node).unwrap();
		let neighbours = deltas((i, j), input[0].len(), input.len());
		for (ni, nj) in neighbours {
			let new_cost = cost + input[nj][ni];
			if *costs.get(&(ni, nj)).unwrap_or(&i64::MAX) > new_cost {
				costs.insert((ni, nj), new_cost);
				*parents.entry((ni, nj)).or_default() = node;
			}
		}
		processed.insert(node);

		// println!("Processed {}", processed.len());
	}

	Ok(*costs.get(&(last_i, last_j)).unwrap())
}

fn deltas(
	(i, j): (usize, usize),
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	let up = if j == 0 { None } else { Some((i, j - 1)) };
	let right = if i >= width - 1 {
		None
	} else {
		Some((i + 1, j))
	};
	let down = if j >= height - 1 {
		None
	} else {
		Some((i, j + 1))
	};
	let left = if i == 0 { None } else { Some((i - 1, j)) };
	[up, right, down, left].into_iter().filter_map(|x| x)
}

fn find_lowest_cost_node(
	costs: &HashMap<(usize, usize), i64>,
	processed: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
	let mut lowest_cost = i64::MAX;
	let mut lowest_cost_node = None;
	for (node, val) in costs {
		if *val < lowest_cost && !processed.contains(node) {
			lowest_cost = *val;
			lowest_cost_node = Some(*node);
		}
	}
	lowest_cost_node
}

#[cfg(test)]
mod tests {
	const EX_1: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 40);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EX_1).unwrap(), 315);
	}
}
