use anyhow::anyhow;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_20/input.txt")?;
	println!("Part 1: {}", solve(&input, 2)?);
	println!("Part 1: {}", solve(&input, 50)?);
	Ok(())
}

fn solve(input: &str, steps: usize) -> anyhow::Result<usize> {
	let mut input_lines = input.lines();
	let enhancing_algorithm: &[u8] = input_lines.next().ok_or_else(|| anyhow!("wtf"))?.as_bytes();

	let mut image = HashMap::<(i64, i64), u8>::new();
	let mut output_image = HashMap::new();

	for (y, line) in input_lines.enumerate() {
		for (x, cell) in line.as_bytes().iter().enumerate() {
			image.insert((x as i64, y as i64), *cell);
		}
	}

	let mut infinite_cell = b'.';
	for _ in 0..steps {
		let min_x = image
			.keys()
			.map(|(x, _)| x)
			.min()
			.ok_or_else(|| anyhow!("wtf"))?
			- 9;
		let min_y = image
			.keys()
			.map(|(_, y)| y)
			.min()
			.ok_or_else(|| anyhow!("wtf"))?
			- 9;
		let max_x = image
			.keys()
			.map(|(x, _)| x)
			.max()
			.ok_or_else(|| anyhow!("wtf"))?
			+ 9;
		let max_y = image
			.keys()
			.map(|(_, y)| y)
			.max()
			.ok_or_else(|| anyhow!("wtf"))?
			+ 9;

		const DELTAS: [(i64, i64); 9] = [
			(-1, -1),
			(0, -1),
			(1, -1),
			(-1, 0),
			(0, 0),
			(1, 0),
			(-1, 1),
			(0, 1),
			(1, 1),
		];

		for y in min_y..=max_y {
			for x in min_x..=max_x {
				let mut bin_num = 0_usize;
				for (idx, (lookup_x, lookup_y)) in DELTAS
					.iter()
					.map(|(dx, dy)| (x + dx, y + dy))
					.rev()
					.enumerate()
				{
					if *image.get(&(lookup_x, lookup_y)).unwrap_or(&infinite_cell) == b'#' {
						bin_num |= 1 << idx;
					}
				}
				output_image.insert((x, y), enhancing_algorithm[bin_num]);
			}
		}

		infinite_cell = if infinite_cell == b'#' {
			enhancing_algorithm[0b_1_1111_1111]
		} else {
			enhancing_algorithm[0]
		};
		std::mem::swap(&mut image, &mut output_image);
	}

	Ok(image.values().filter(|cell| **cell == b'#').count())
}

#[cfg(test)]
mod tests {
	const EX_1: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

	#[test]
	fn part_1() {
		assert_eq!(super::solve(EX_1, 2).unwrap(), 35);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::solve(EX_1, 50).unwrap(), 3351);
	}
}
