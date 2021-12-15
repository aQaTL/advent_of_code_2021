extern crate itertools;
extern crate nom;

use anyhow::bail;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

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
	dijkstra(input)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let mut input: Vec<Vec<i64>> = input
		.lines()
		.map(|line| line.bytes().map(|x| (x - b'0') as i64).collect_vec())
		.collect_vec();
	let side_size = input.len();

	for c in 1..5 {
		for j in 0..side_size {
			let mut vec = Vec::with_capacity(side_size * 5);
			let next = &input[(c - 1) * side_size + j];
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

	for j in 0..(side_size * 5) {
		for c in 1..5 {
			for i in 0..side_size {
				let mut next = input[j][(c - 1) * side_size + i] + 1;
				if next > 9 {
					next -= 9;
				}
				input[j].push(next);
			}
		}
	}

	dijkstra(input)
}

fn dijkstra(input: Vec<Vec<i64>>) -> anyhow::Result<i64> {
	let side_size = input.len();

	let mut priority_queue = BinaryHeap::<Reverse<Cell>>::new();
	let mut costs = HashMap::<(usize, usize), i64>::new();

	priority_queue.push(Reverse(Cell { p: (0, 0), cost: 0 }));

	while let Some(Reverse(Cell { p, cost })) = priority_queue.pop() {
		if p == (side_size - 1, side_size - 1) {
			return Ok(cost);
		}

		if cost > *costs.get(&p).unwrap_or(&i64::MAX) {
			continue;
		}

		let neighbours = deltas(p, side_size, side_size);
		for (ni, nj) in neighbours {
			let new_cost = cost + input[nj][ni];
			if *costs.get(&(ni, nj)).unwrap_or(&i64::MAX) > new_cost {
				priority_queue.push(Reverse(Cell {
					p: (ni, nj),
					cost: new_cost,
				}));
				costs.insert((ni, nj), new_cost);
			}
		}
	}

	bail!("Exit node not found")
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Cell {
	p: (usize, usize),
	cost: i64,
}

impl Ord for Cell {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&other.cost)
	}
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
