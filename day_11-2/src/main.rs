use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Octopus {
	energy: u32,
	has_flashed: bool,
	has_incremented: bool,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let mut octomap = init_octomap(input_lines);
	let nb_octopuses = octomap.len() * octomap.get(0).unwrap().len();
	let mut nb_flashes = 0;
	let mut step = 1;
	print_octomap(&octomap);
	loop {
		reset_flashes_and_increments(&mut octomap);
		let curr_flash = increment_energy(&mut octomap, 0, 0, false);
		println!("-------- Step {} --------", step);
		print_octomap(&octomap);
		println!("curr_flash: {}", curr_flash);
		if curr_flash as usize == nb_octopuses {
			println!("At step {} everybody flashed!!", step);
			break;
		}
		nb_flashes += curr_flash;
		step += 1;
	}
	println!("Total flashes before sync: {}", nb_flashes);
}

fn init_octomap(input_lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<Octopus>> {
	let mut octomap: Vec<Vec<Octopus>> = Vec::new();

	for line in input_lines.flatten() {
		if line.is_empty() == false {
			let octo_in_line: Vec<Octopus> = line.chars()
				.map(|s| Octopus {
					energy: s.to_string().parse().unwrap(),
					has_incremented: false,
					has_flashed: false,
				})
				.collect();
			octomap.push(octo_in_line);
		}
	}
	octomap
}

fn reset_flashes_and_increments(octomap: &mut Vec<Vec<Octopus>>) {
	for line in octomap {
		for octopus in line {
			octopus.has_incremented = false;
			octopus.has_flashed = false;
		}
	}
}

fn increment_energy(octomap: &mut Vec<Vec<Octopus>>,
	row: usize,
	col: usize,
	neighbor_flashed: bool) -> u32 {
	let mut curr_octopus = octomap.get_mut(row).unwrap().get_mut(col).unwrap();
	if curr_octopus.has_flashed {
		return 0
	}
	else if neighbor_flashed {
		curr_octopus.energy += 1;
		if curr_octopus.energy > 9 {
			curr_octopus.has_flashed = true;
			curr_octopus.has_incremented = true;
			curr_octopus.energy = 0;
			return 1 + spread(octomap, row, col, true);
		}
		else {
			return spread(octomap, row, col, false);
		}
	}
	else if !curr_octopus.has_incremented {
		curr_octopus.energy += 1;
		curr_octopus.has_incremented = true;
		if curr_octopus.energy > 9 {
			curr_octopus.has_flashed = true;
			curr_octopus.energy = 0;
			return 1 + spread(octomap, row, col, true);
		}
		else {
			return spread(octomap, row, col, false);
		}
	}
	else {
		return 0
	}
}

fn spread(octomap: &mut Vec<Vec<Octopus>>,
	row: usize,
	col: usize,
	neighbor_flashed: bool) -> u32 {
	let row_size = octomap.len();
	let col_size = octomap.get(0).unwrap().len();
	let mut nb_flashes = 0;

	if row > 0 && col > 0 {
		nb_flashes += increment_energy(octomap, row - 1, col - 1, neighbor_flashed);
	}
	if row > 0 {
		nb_flashes += increment_energy(octomap, row - 1, col, neighbor_flashed);
	}
	if row > 0 && col + 1 < col_size  {
		nb_flashes += increment_energy(octomap, row - 1, col + 1, neighbor_flashed);
	}
	if col > 0  {
		nb_flashes += increment_energy(octomap, row, col - 1, neighbor_flashed);
	}
	if col + 1 < col_size  {
		nb_flashes += increment_energy(octomap, row, col + 1, neighbor_flashed);
	}
	if row + 1 < row_size && col > 0 {
		nb_flashes += increment_energy(octomap, row + 1, col - 1, neighbor_flashed);
	}
	if row + 1 < row_size {
		nb_flashes += increment_energy(octomap, row + 1, col, neighbor_flashed);
	}
	if row + 1 < row_size && col + 1 < col_size  {
		nb_flashes += increment_energy(octomap, row + 1, col + 1, neighbor_flashed);
	}
	nb_flashes
}

fn print_octomap(octomap: &Vec<Vec<Octopus>>) {
	for line in octomap {
		for octo in line {
			print!("{}", octo.energy);
		}
		println!("");
	}
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
