use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct PairInsertion {
	left: String,
	right: String,
}

const CYCLES: u32 = 40;

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let (poly_template, insertion_rules) = init_inputs(input_lines);
	let mut poly_cache = init_cache(&insertion_rules);
	let mut pair_count: HashMap<String, usize> = HashMap::new();
	for pair in &poly_template {
		pair_count = merge_counts(
			pair_count,
			expand_pair(pair, &insertion_rules, CYCLES, &mut poly_cache));
	}
	
	let template_last_char = poly_template.last().unwrap().chars().nth(1).unwrap();
	let chars_count = get_chars_count(pair_count, template_last_char);
	let max_el = get_max_el(&chars_count);
	let min_el = get_min_el(&chars_count);
	println!("Max: {} - min: {} = {}", max_el, min_el, max_el - min_el);
}

fn init_inputs(input_lines: io::Lines<io::BufReader<File>>)
	-> (Vec<String>,
		HashMap<String, PairInsertion>) {
	let mut save_poly_template = true;
	let mut poly_template: Vec<String> = Vec::new();
	let mut insertion_rules: HashMap<String, PairInsertion> = HashMap::new();

	for line in input_lines.flatten() {
		if line.is_empty() {
			save_poly_template = false;
		}
		else if save_poly_template {
			for i in 0..line.len() - 1 {
				poly_template.push(line[i..i + 2].to_string());
			}
		}
		else {
			let pair: Vec<&str> = line.split(" -> ")
				.collect();
			let initial_pair = pair.get(0).unwrap();
			let pair_first = pair.get(0).unwrap().chars().next().unwrap();
			let pair_last = pair.get(0).unwrap().chars().nth(1).unwrap();
			let to_insert = pair.get(1).unwrap().chars().next().unwrap();
			let pair_insertion = PairInsertion {
				left: format!("{}{}", pair_first, to_insert),
				right: format!("{}{}", to_insert, pair_last),
			};
			insertion_rules.insert(initial_pair.to_string(), pair_insertion);
		}
	}
	(poly_template, insertion_rules)
}

fn init_cache(insertion_rules: &HashMap<String, PairInsertion>)
	-> HashMap<(String, u32), HashMap<String, usize>> {
	let mut poly_cache: HashMap<(String, u32), HashMap<String, usize>> = HashMap::new();

	for insertion in insertion_rules {
		let mut el_count: HashMap<String, usize> = HashMap::new();
		let curr_pair = el_count.entry(insertion.1.left.clone()).or_insert(0);
		*curr_pair += 1;
		let curr_pair = el_count.entry(insertion.1.right.clone()).or_insert(0);
		*curr_pair += 1;
		poly_cache.insert((insertion.0.to_string(), 1), el_count);
	}
	poly_cache
}

fn expand_pair(pair: &str,
	insertion_rules: &HashMap<String, PairInsertion>,
	cycles: u32,
	poly_cache: &mut HashMap<(String, u32), HashMap<String, usize>>)
	-> HashMap<String, usize> {
		if let Some(cached_poly) = poly_cache.get(&(pair.to_string(), cycles)) {
			cached_poly.clone()
		}
	else {
		let pair_rules = insertion_rules.get(pair).unwrap();
		let pair_count_left = expand_pair(&pair_rules.left, insertion_rules, cycles - 1, poly_cache);
		let pair_count_right = expand_pair(&pair_rules.right, insertion_rules, cycles - 1, poly_cache);
		let pair_count = merge_counts(pair_count_left, pair_count_right);
		poly_cache.insert((pair.to_string(), cycles), pair_count.clone());
		pair_count
	}
}

fn merge_counts(count_left: HashMap<String, usize>, count_right: HashMap<String, usize>)
	-> HashMap<String, usize> {
	let mut count: HashMap<String, usize> = count_left;
	for (key, value) in count_right {
		if let Some(element) = count.get_mut(&key) {
			*element += value;
		} else {
			count.insert(key, value);
		}
	}
	count
}

fn get_chars_count(pair_count: HashMap<String, usize>, last_char: char)
	-> HashMap<char, usize> {
	let mut chars_count: HashMap<char, usize> = HashMap::new();

	for pair in pair_count {
		let pair_left_char = pair.0.chars().next().unwrap();
		let left_char = chars_count.entry(pair_left_char).or_insert(0);
		*left_char += pair.1;
	}
	let last_char = chars_count.entry(last_char).or_insert(0);
	*last_char += 1;
	chars_count
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
