use std::sync::atomic::{AtomicUsize, Ordering};
use std::boxed::Box;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const RADIX: u32 = 10;

#[derive(Debug, PartialEq, Eq)]
struct Pair {
	id: usize,
	left: Box<PairElement>,
	right: Box<PairElement>,
}

#[derive(Debug, PartialEq, Eq)]
enum PairElement {
	Number(u32),
	Pair(Pair),
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	let mut numbers: Vec<Pair> = Vec::new();
	
	for line in input_lines.flatten() {
		if !line.is_empty() {
			println!("{}", line);
			// Reset the char counter to 0
			consume_next_char(&line, true);
			// We don't pass the first '['
			numbers.push(parse_fish_number(&line[1..], next_id()));
			println!("{:#?}", numbers.last());
			if numbers.len() == 2 {
				let right = numbers.pop().unwrap();
				let left = numbers.pop().unwrap();
				numbers.push(reduce(sum_numbers(left, right)));
			}
			println!("{:#?}", numbers.last());
		}
	}
}

fn parse_fish_number(fish_number: &str, id: usize) -> Pair {
	let left: PairElement;
	let right: PairElement;

	left = get_next_number(fish_number);
	// Remove the ','
	consume_next_char(fish_number, false);
	
	right = get_next_number(fish_number);

	// Remove the ']'
	consume_next_char(fish_number, false);
	Pair {
		id,
		left: Box::new(left),
		right: Box::new(right),
	}
}

fn get_next_number(fish_number: &str) -> PairElement {
	let first_char = consume_next_char(fish_number, false);
	if first_char == '[' {
		PairElement::Pair(parse_fish_number(fish_number, next_id()))
	} else {
		PairElement::Number(first_char.to_digit(RADIX).unwrap())
	}
}


fn consume_next_char(s: &str, reset: bool) -> char {
	static INDEX: AtomicUsize = AtomicUsize::new(0);
	
	if reset {
		INDEX.store(0, Ordering::SeqCst);
		'R'
	} else {
		INDEX.fetch_add(1, Ordering::SeqCst);
		s.chars().nth(INDEX.load(Ordering::SeqCst) - 1).unwrap()
	}
	
}

fn sum_numbers(left: Pair, right: Pair) -> Pair {
	println!("SUMMING TWO LAST PAIRS ----------");
	Pair {
		id: 0,
		left: Box::new(PairElement::Pair(left)),
		right: Box::new(PairElement::Pair(right)),
	}
}

fn reduce(number: Pair, level: u32) -> Pair {
	if matches!(*number.left, PairElement::Pair(_)) {
		reduce(*number.left, level + 1);
	}
	
	number
}

fn next_id() -> usize {
	static INDEX: AtomicUsize = AtomicUsize::new(0);

	INDEX.fetch_add(1, Ordering::SeqCst);
	INDEX.load(Ordering::SeqCst) - 1
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
