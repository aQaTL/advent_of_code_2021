use std::collections::{HashSet, VecDeque};
use std::slice::SliceIndex;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_9/input.txt")?;
	let SmokeFlowModel {
		risk_level_sum,
		basin_size_product,
	} = smoke_flow_model(&input)?;
	println!("Part 1: {}", risk_level_sum);
	println!("Part 2: {}", basin_size_product);
	Ok(())
}

struct SmokeFlowModel {
	risk_level_sum: i64,
	basin_size_product: i64,
}

fn smoke_flow_model(input: &str) -> anyhow::Result<SmokeFlowModel> {
	let mut grid = Vec::new();
	for line in input.lines() {
		let mut grid_line = Vec::new();
		for cell_idx in 0..line.len() {
			let cell = line[cell_idx..(cell_idx + 1)].parse::<i64>()?;
			grid_line.push(cell);
		}
		grid.push(grid_line);
	}

	let mut risk_level_sum = 0;
	let mut basin_sizes = Vec::new();

	for j in 0..(grid.len() as i64) {
		for i in 0..(grid[j as usize].len() as i64) {
			let up = grid
				.geti(j + 1)
				.and_then(|line| line.geti(i))
				.map(|x| *x)
				.unwrap_or(i64::MAX);
			let right = grid
				.geti(j)
				.and_then(|line| line.geti(i + 1))
				.map(|x| *x)
				.unwrap_or(i64::MAX);
			let down = grid
				.geti(j - 1)
				.and_then(|line| line.geti(i))
				.map(|x| *x)
				.unwrap_or(i64::MAX);
			let left = grid
				.geti(j)
				.and_then(|line| line.geti(i - 1))
				.map(|x| *x)
				.unwrap_or(i64::MAX);
			let cell = grid[j as usize][i as usize];
			if cell < up && cell < right && cell < left && cell < down {
				risk_level_sum += cell + 1;
				basin_sizes.push(basin_size(&grid, (i, j)));
			}
		}
	}

	basin_sizes.sort_unstable_by_key(|x| std::cmp::Reverse(*x));

	Ok(SmokeFlowModel {
		risk_level_sum,
		basin_size_product: basin_sizes[0..3].iter().product(),
	})
}

fn basin_size(grid: &Vec<Vec<i64>>, point: (i64, i64)) -> i64 {
	let mut explored = HashSet::from([point]);
	let mut queue = VecDeque::from([point]);
	let mut count = 0;

	while let Some((i, j)) = queue.pop_front() {
		if let Some(x) = grid.geti(j).and_then(|line| line.geti(i)) {
			if *x >= 9 {
				continue;
			}
			count += 1;

			let up = (i, j + 1);
			let right = (i + 1, j);
			let down = (i, j - 1);
			let left = (i - 1, j);

			if !explored.contains(&up) {
				explored.insert(up);
				queue.push_back(up);
			}
			if !explored.contains(&right) {
				explored.insert(right);
				queue.push_back(right);
			}
			if !explored.contains(&down) {
				explored.insert(down);
				queue.push_back(down);
			}
			if !explored.contains(&left) {
				explored.insert(left);
				queue.push_back(left);
			}
		}
	}

	count
}

trait GetI64Idx<T> {
	fn geti(&self, index: i64) -> Option<&<usize as SliceIndex<[T]>>::Output>;
}

impl<T> GetI64Idx<T> for Vec<T> {
	fn geti(&self, index: i64) -> Option<&<usize as SliceIndex<[T]>>::Output> {
		if index < 0 {
			return None;
		}
		self.get(index as usize)
	}
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

	#[test]
	fn part_1() {
		assert_eq!(super::smoke_flow_model(EX_1).unwrap().risk_level_sum, 15);
	}

	#[test]
	fn part_2() {
		assert_eq!(
			super::smoke_flow_model(EX_1).unwrap().basin_size_product,
			1134
		);
	}
}
