use anyhow::bail;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_4/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn score(board: &Board, num: u32) -> u32 {
	board
		.b
		.iter()
		.filter(|x| !x.marked)
		.map(|x| x.num)
		.sum::<u32>()
		* num
}

fn part_1(input: &str) -> anyhow::Result<u32> {
	let (
		_,
		Input {
			numbers,
			mut boards,
		},
	) = parse_input(input).unwrap();

	for num in numbers {
		boards
			.iter_mut()
			.flat_map(|b| b.b.iter_mut())
			.filter(|x| x.num == num)
			.for_each(|x: &mut BoardCell| x.marked = true);

		for board in &boards {
			// horizontal win
			if board
				.b
				.chunks(5)
				.find(|chunk| chunk.iter().all(|x| x.marked))
				.is_some()
			{
				return Ok(score(board, num));
			}
			// vertical win
			for chunk in (0..5)
				.flat_map(|column| (0..5).map(move |row| row * 5 + column))
				.chunks(5)
				.into_iter()
			{
				if !chunk.into_iter().all(|idx| board.b[idx].marked) {
					continue;
				}
				return Ok(score(board, num));
			}
		}
	}

	bail!("winning board not found")
}

fn part_2(input: &str) -> anyhow::Result<u32> {
	let (
		_,
		Input {
			numbers,
			mut boards,
		},
	) = parse_input(input).unwrap();

	let (mut won_boards, boards_count) = (0, boards.len());

	for num in numbers {
		boards
			.iter_mut()
			.flat_map(|b| b.b.iter_mut())
			.filter(|x| x.num == num)
			.for_each(|x: &mut BoardCell| x.marked = true);

		'l: for board in &mut boards {
			if board.won {
				continue;
			}
			// horizontal win
			if board
				.b
				.chunks(5)
				.find(|chunk| chunk.iter().all(|x| x.marked))
				.is_some()
			{
				board.won = true;
				won_boards += 1;
				if won_boards == boards_count {
					return Ok(score(board, num));
				}
				continue 'l;
			}
			// vertical win
			for chunk in (0..5)
				.flat_map(|column| (0..5).map(move |row| row * 5 + column))
				.chunks(5)
				.into_iter()
			{
				if !chunk.into_iter().all(|idx| board.b[idx].marked) {
					continue;
				}
				board.won = true;
				won_boards += 1;
				if won_boards == boards_count {
					return Ok(score(board, num));
				}
				continue 'l;
			}
		}
	}

	bail!("winning board not found")
}

struct Input {
	numbers: Vec<u32>,
	boards: Vec<Board>,
}

struct Board {
	b: Vec<BoardCell>,
	won: bool,
}

struct BoardCell {
	num: u32,
	marked: bool,
}

fn parse_input(input: &str) -> IResult<&str, Input> {
	map(
		tuple((
			terminated(
				many1(terminated(nom::character::complete::u32, opt(char(',')))),
				tag("\n\n"),
			),
			many1(terminated(parse_board, multispace0)),
		)),
		|(numbers, boards)| Input { numbers, boards },
	)(input)
}

fn parse_board(input: &str) -> IResult<&str, Board> {
	map(
		many1(delimited(
			space0,
			map(nom::character::complete::u32, |num| BoardCell {
				num,
				marked: false,
			}),
			alt((space1, tag("\n"))),
		)),
		|b| Board { b, won: false },
	)(input)
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

	#[test]
	fn part_1() -> anyhow::Result<()> {
		assert_eq!(super::part_1(EX_1)?, 4512);
		Ok(())
	}

	#[test]
	fn part_2() -> anyhow::Result<()> {
		assert_eq!(super::part_2(EX_1)?, 1924);
		Ok(())
	}
}
