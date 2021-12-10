fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_10/input.txt")?;
	let SyntaxScore {
		total_syntax_error_score,
		autocomplete_middle_score,
	} = syntax_score(&input)?;
	println!("Part 1: {}", total_syntax_error_score);
	println!("Part 2: {}", autocomplete_middle_score);
	Ok(())
}

struct SyntaxScore {
	total_syntax_error_score: u64,
	autocomplete_middle_score: u64,
}

fn syntax_score(input: &str) -> anyhow::Result<SyntaxScore> {
	let mut total_syntax_error_score = 0;
	let mut autocomplete_scores = Vec::new();

	let mut queue = Vec::new();

	'lines: for line in input.lines() {
		queue.clear();
		for c in line.as_bytes() {
			match *c {
				b'{' | b'[' | b'<' | b'(' => queue.push(*c),
				c => {
					let opening_char = match queue.pop() {
						Some(v) => v,
						None => continue 'lines,
					};

					let score = match (opening_char, c) {
						(b'(', b')') | (b'[', b']') | (b'{', b'}') | (b'<', b'>') => continue,
						(_, b')') => 3,
						(_, b']') => 57,
						(_, b'}') => 1197,
						(_, b'>') => 25137,
						_ => anyhow::bail!("wtf: ({}, {})", opening_char as char, c as char),
					};
					total_syntax_error_score += score;
					continue 'lines;
				}
			}
		}

		let mut autocomplete_score = 0;
		while let Some(c) = queue.pop() {
			autocomplete_score *= 5;
			match c {
				b'(' => autocomplete_score += 1,
				b'[' => autocomplete_score += 2,
				b'{' => autocomplete_score += 3,
				b'<' => autocomplete_score += 4,
				_ => anyhow::bail!("wtf: {}", c as char),
			}
		}
		autocomplete_scores.push(autocomplete_score);
	}
	autocomplete_scores.sort_unstable();

	Ok(SyntaxScore {
		total_syntax_error_score,
		autocomplete_middle_score: autocomplete_scores[autocomplete_scores.len() / 2],
	})
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

	#[test]
	fn part_1() {
		assert_eq!(
			super::syntax_score(EX_1).unwrap().total_syntax_error_score,
			26397
		);
	}

	#[test]
	fn part_2() {
		assert_eq!(
			super::syntax_score(EX_1).unwrap().autocomplete_middle_score,
			288957
		);
	}
}
