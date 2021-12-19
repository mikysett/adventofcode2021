use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct PairInsertion {
	first: char,
	second: char,
	insert: char,
}

const CYCLES: u32 = 10;

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let (mut poly_template, insertion_rules) = init_inputs(input_lines);
	
	println!("Poly template: {:?}", poly_template);

	for i in 1..CYCLES + 1 {
		do_step(&mut poly_template, &insertion_rules);
		println!("Step {}: size of polymer: {}", i, poly_template.len());
	}

	let el_count = count_elements(&poly_template);
	println!("Count of elements:\n{:?}", el_count);
	let max_el = get_max_el(&el_count);
	let min_el = get_min_el(&el_count);
	println!("Max: {} - min: {} = {}", max_el, min_el, max_el - min_el);
}

fn init_inputs(input_lines: io::Lines<io::BufReader<File>>) -> (Vec<char>, Vec<PairInsertion>) {
	let mut save_poly_template = true;
	let mut poly_template: Vec<char> = Vec::new();
	let mut insertion_rules: Vec<PairInsertion> = Vec::new();

	for line in input_lines.flatten() {
		if line.is_empty() {
			save_poly_template = false;
		}
		else if save_poly_template {
			poly_template = line.chars().collect();
		}
		else {
			let pair: Vec<&str> = line.split(" -> ")
				.collect();
			let pair = PairInsertion {
				first: pair.get(0).unwrap().chars().next().unwrap(),
				second: pair.get(0).unwrap().chars().nth(1).unwrap(),
				insert: pair.get(1).unwrap().chars().next().unwrap(),
			};
			insertion_rules.push(pair);
		}
	}
	(poly_template, insertion_rules)
}

fn do_step(poly_template: &mut Vec<char>, insertion_rules: &[PairInsertion]) {
	let mut i = 0;
	let mut template_len = poly_template.len();
	loop {
		if i == template_len - 1 {
			break;
		}
		let curr_el = poly_template.get(i).unwrap();
		let next_el = poly_template.get(i + 1).unwrap();
		if let Some(to_insert) = find_pair(insertion_rules, *curr_el, *next_el) {
			poly_template.insert(i + 1, to_insert);
			i += 2;
			template_len += 1;
		} else {
			i += 1;
		}
	}
}

fn find_pair(insertion_rules: &[PairInsertion], curr_el: char, next_el: char)
-> Option<char> {
	for rule in insertion_rules {
		if rule.first == curr_el && rule.second == next_el {
			return Some(rule.insert);
		}
	}
	None
}

fn count_elements(poly_template: &[char]) -> HashMap<char, usize> {
	let mut el_count: HashMap<char, usize> = HashMap::new();

	for el in poly_template {
		let sgl_el = el_count.entry(*el).or_insert(0);
		*sgl_el += 1;
	}
	el_count
}

fn get_max_el(el_count: &HashMap<char, usize>) -> usize {
	let mut max: usize = 0;
	for val in el_count.values() {
		if *val > max {
			max = *val;
		}
	}
	max
}

fn get_min_el(el_count: &HashMap<char, usize>) -> usize {
	let mut min: usize = 0;
	for val in el_count.values() {
		if *val < min || min == 0 {
			min = *val;
		}
	}
	min
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
