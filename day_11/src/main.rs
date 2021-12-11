fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_11/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u64> {
	let mut input: Vec<u8> = input
		.lines()
		.flat_map(|line| line.trim().as_bytes())
		.map(|x| x - b'0')
		.collect();
	let mut flashed: Vec<bool> = vec![false; input.len()];

	let mut flash_count = 0;

	for _ in 0..100 {
		flashed.iter_mut().for_each(|x| *x = false);

		for j in 0..10 {
			for i in 0..10 {
				input[j * 10 + i] += 1;
				if input[j * 10 + i] > 9 && !flashed[j * 10 + i] {
					flashed[j * 10 + i] = true;
					flash_count += 1;
					flash_count += flash(&mut input, &mut flashed, (i, j));
				}
			}
		}

		input.iter_mut().filter(|x| **x > 9).for_each(|x| *x = 0);
	}

	Ok(flash_count)
}

fn part_2(input: &str) -> anyhow::Result<u64> {
	let mut input: Vec<u8> = input
		.lines()
		.flat_map(|line| line.trim().as_bytes())
		.map(|x| x - b'0')
		.collect();
	let mut flashed: Vec<bool> = vec![false; input.len()];

	for step in 1.. {
		flashed.iter_mut().for_each(|x| *x = false);

		for j in 0..10 {
			for i in 0..10 {
				input[j * 10 + i] += 1;
				if input[j * 10 + i] > 9 && !flashed[j * 10 + i] {
					flashed[j * 10 + i] = true;
					flash(&mut input, &mut flashed, (i, j));
				}
			}
		}
		if flashed.iter().all(|x| *x) {
			return Ok(step);
		}
		input.iter_mut().filter(|x| **x > 9).for_each(|x| *x = 0);
	}

	unreachable!()
}

fn flash(input: &mut Vec<u8>, flashed: &mut Vec<bool>, (i, j): (usize, usize)) -> u64 {
	static DELTAS: [(isize, isize); 8] = [
		(0, -1),
		(1, -1),
		(1, 0),
		(1, 1),
		(0, 1),
		(-1, 1),
		(-1, 0),
		(-1, -1),
	];

	let mut flash_count = 0;
	for (di, dj) in DELTAS
		.iter()
		.map(|(di, dj)| (i as isize + *di, j as isize + *dj))
	{
		if di < 0 || dj < 0 || di >= 10 || dj >= 10 {
			continue;
		}
		let (di, dj) = (di as usize, dj as usize);

		input[dj * 10 + di] += 1;
		if !flashed[dj * 10 + di] && input[dj * 10 + di] > 9 {
			flashed[dj * 10 + di] = true;
			flash_count += 1;
			flash_count += flash(input, flashed, (di, dj));
		}
	}

	flash_count
}

#[cfg(test)]
mod tests {
	const EX_1: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

	#[test]
	fn part_1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 1656);
	}

	#[test]
	fn part_2() {
		assert_eq!(super::part_2(EX_1).unwrap(), 195);
	}
}
