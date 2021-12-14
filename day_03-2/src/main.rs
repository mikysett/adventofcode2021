use std::fs;

#[derive(PartialEq, Eq)]
enum Rating {
	OxygenGeneratior,
	CO2Scrubber,
}

fn main() {
	let input = fs::read_to_string("input")
		.expect("Can't open input file");
	let binaries: Vec<&str> = input.split_whitespace().collect();
	let oxygen_generator = calculate_rating(&binaries, Rating::OxygenGeneratior);
	let co2scrubber = calculate_rating(&binaries, Rating::CO2Scrubber);
	println!("oxygen: {}", oxygen_generator);
	println!("co2   : {}", co2scrubber);
	println!("life support: {}", oxygen_generator * co2scrubber);
}

fn calculate_rating(binaries: &Vec<&str>, rating: Rating) -> u32 {
	let mut binaries = binaries.clone();
	
	for bit_pos in 0..binaries[0].len() {
		let expected_bit = get_expected_bit(&binaries, bit_pos, &rating);
		remove_unmatched_bin(&mut binaries, expected_bit, bit_pos);
	}
	binary_to_int(binaries[0])
}

fn get_expected_bit(binaries: &Vec<&str>, bit_pos: usize, rating: &Rating) -> char {
	let mut accumulator = 0;

	for bin in binaries.iter() {
		match bin.chars().nth(bit_pos) {
			Some(bit) => {
				if bit == '1' {
					accumulator += 1;
				}
				else if bit == '0' {
					accumulator -= 1;
				}
			},
			None => continue
		}
	}
	if accumulator >= 0 {
		if rating == &Rating::OxygenGeneratior { '1' } else { '0' }
	}
	else {
		if rating == &Rating::OxygenGeneratior { '0' } else { '1' }
	}
}

fn remove_unmatched_bin(binaries: &mut Vec<&str>, expected_bit: char, bit_pos: usize) {
	let mut bin_pos = 0;

	while bin_pos < binaries.len() {
		if binaries.len() <= 1 {
			break;
		}
		let curr_bit = match binaries[bin_pos].chars().nth(bit_pos) {
			Some(bit_val) => bit_val,
			None => break
		};
		if curr_bit != expected_bit {
			binaries.remove(bin_pos);
		}
		else {
			bin_pos += 1;
		}
	}
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
