use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum BracketType {
	Round,	// ()
	Square, // []
	Curly,	// {}
	Angle,	// <>
}

enum LineStatus {
	Ok(usize),
	Incomplete,
	Error(BracketType),
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");

	let mut scores: Vec<u64> = Vec::new();
	for line in input_lines.flatten() {
		if !line.is_empty() {
			let mut autocomplete: Vec<BracketType> = Vec::new();
			if let LineStatus::Incomplete = parse_line(&mut autocomplete, &line, 0) {
				println!("Autocomplete: {:?}", autocomplete);
				scores.push(calc_score(autocomplete));
			}
		}
	}
	scores.sort();
	println!("Scores: {:?}", scores);
	println!("Middle score: {}", scores.get(scores.len() / 2).unwrap());
}

fn parse_line(autocomplete: &mut Vec<BracketType>, line: &str, i: usize)
	-> LineStatus {
	if i >= line.len() {
		return LineStatus::Ok(i);
	}

	let curr_bracket = get_bracket_type(line.chars().nth(i).unwrap());
	let mut i = i;

	loop {
		if i + 1 >= line.len() {
			autocomplete.push(curr_bracket);
			return LineStatus::Incomplete;
		}
		let next_char = line.chars().nth(i + 1).unwrap();
		let next_bracket = get_bracket_type(next_char);
		if is_closing_bracket(next_char) {
			if curr_bracket != next_bracket {
				return LineStatus::Error(next_bracket);
			}
			else if i + 2 < line.len()
				&& is_closing_bracket(line.chars().nth(i + 2).unwrap()) {
				return LineStatus::Ok(i + 1);
			}
			else {
				return parse_line(autocomplete, line, i + 2);
			}
		}
		else {
			match parse_line(autocomplete, line, i + 1) {
				LineStatus::Ok(index) => {
					i = index;
				},
				LineStatus::Incomplete => {
					autocomplete.push(curr_bracket);
					return LineStatus::Incomplete
				}
				other => return other,
			}
		}
	}
}

fn get_bracket_type(c: char) -> BracketType {
	if c == '(' || c == ')' {
		BracketType::Round
	}
	else if c == '[' || c == ']' {
		BracketType::Square
	}
	else if c == '{' || c == '}' {
		BracketType::Curly
	}
	else if c == '<' || c == '>' {
		BracketType::Angle
	}
	else {
		panic!("Invalid character!!!")
	}
}

fn is_closing_bracket(c: char) -> bool {
	if c == ')' || c == ']' || c == '}' || c == '>' {
		return true
	}
	false
}

fn calc_score(autocomplete: Vec<BracketType>) -> u64 {
	let mut score: u64 = 0;
	let mut scores: HashMap<BracketType, u32> = HashMap::new();
	scores.entry(BracketType::Round).or_insert(1);
	scores.entry(BracketType::Square).or_insert(2);
	scores.entry(BracketType::Curly).or_insert(3);
	scores.entry(BracketType::Angle).or_insert(4);

	for bracket in autocomplete {
		score = score * 5 + *scores.entry(bracket).or_insert(0) as u64;
	}
	score
}

fn calc_total_score(errors: &HashMap<BracketType, u32>) -> u32 {
	let mut total_score = 0;
	let mut scores: HashMap<BracketType, u32> = HashMap::new();
	scores.entry(BracketType::Round).or_insert(3);
	scores.entry(BracketType::Square).or_insert(57);
	scores.entry(BracketType::Curly).or_insert(1197);
	scores.entry(BracketType::Angle).or_insert(25137);

	for (bracket, nb_err) in errors {
		println!("score for {:?} bracket: {}",
			bracket,
			*scores.entry(*bracket).or_insert(0) * nb_err);
		total_score += *scores.entry(*bracket).or_insert(0) * nb_err;
	}
	total_score
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
