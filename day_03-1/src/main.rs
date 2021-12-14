use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct BitStatus {
	yes: u32,
	no: u32,
}

// Program dynamically infer the number of bits up to 100 bits per line

fn main() {
	let input = fs::read_to_string("input")
		.expect("Can't open input file");
	let binaries = input.split('\n');
	let mut bit_statuses = HashMap::new();

	for bin in binaries {
		update_statuses(&bin, &mut bit_statuses);
	}

	let gamma = get_gamma(&bit_statuses);
	let epsilon = get_epsilon_from_gamma(&gamma);

	println!("gamma: {}, epsilon: {}", gamma, epsilon);
	let gamma = binary_to_int(&gamma);
	let epsilon = binary_to_int(&epsilon);
	println!("gamma: {}, epsilon: {}", gamma, epsilon);
	println!("Power consumption: {}", gamma * epsilon);
}

fn update_statuses(bin: &str, bit_statuses: &mut HashMap<u32, BitStatus>) {
	let mut pos: u32 = 0;
	
	for c in bin.chars() {
		let curr_bit = bit_statuses.entry(pos)
			.or_insert(BitStatus {yes: 0, no: 0});
		if c == '1' {
			curr_bit.yes += 1;
		}
		else if c == '0' {
			curr_bit.no += 1;
		}
		pos += 1;
	}
}

fn get_gamma(bit_statuses: &HashMap<u32, BitStatus>) -> String {
	let mut gamma = String::new();
	
	for i in 0..100 {
		let curr_bit = match bit_statuses.get(&i) {
			Some(bit_status) => bit_status,
			None => break,
		};
		if curr_bit.yes >= curr_bit.no {
			gamma.push('1');
		}
		else {
			gamma.push('0')
		}
	}
	gamma
}

fn get_epsilon_from_gamma(gamma: &String) -> String {
	let mut epsilon = String::new();

	for c in gamma.chars() {
		if c == '1' {
			epsilon.push('0');
		}
		else {
			epsilon.push('1');
		}
	}
	epsilon
}

fn binary_to_int(bin_str: &str) -> u32 {
	let base: u32 = 2;
	let mut power = bin_str.len() as u32;
	let mut nb = 0;
	for bit in bin_str.chars() {
		power -= 1;
		if bit == '1' {
			nb += base.pow(power);
		}
	}
	nb
}
