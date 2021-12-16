use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	let mut low_point_sum = 0;
	
	let heightmap = init_heightmap(input_lines);
	for row in 0..heightmap.len() {
		for col in 0..heightmap.get(row).unwrap().len() {
			let curr_point = heightmap.get(row).unwrap().get(col).unwrap();
			if is_low_point(&heightmap, &curr_point, row, col) {
				println!("low point: {}", curr_point);
				low_point_sum += curr_point + 1;
			}
		}
	}
	println!("Low point sum: {}", low_point_sum);
}

fn init_heightmap(input_lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
	let mut heightmap: Vec<Vec<u32>> = Vec::new();

	for line in input_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() == false {
				let nb_in_lines: Vec<u32> = sgl_line.chars()
					.map(|s| s.to_string().parse().unwrap())
					.collect();
				heightmap.push(nb_in_lines);
			}
		}
	}
	heightmap
}

fn is_low_point(heightmap: &Vec<Vec<u32>>, curr_point: &u32, row: usize, col: usize) -> bool {
	let top;
	let right;
	let bottom;
	let left;
	
	if row > 0 {
		top = get_point(&heightmap, row - 1, col);
	} else {
		top = None;
	}
	right = get_point(&heightmap, row, col + 1);
	bottom = get_point(&heightmap, row + 1, col);
	if col > 0 {
		left = get_point(&heightmap, row, col - 1);
	} else {
		left = None;
	}
	
	if let Some(nb) = top {
		if nb <= curr_point { return false };
	}
	if let Some(nb) = right {
		if nb <= curr_point { return false };
	}
	if let Some(nb) = bottom {
		if nb <= curr_point { return false };
	}
	if let Some(nb) = left {
		if nb <= curr_point { return false };
	}
	true
}

fn get_point(heightmap: &Vec<Vec<u32>>, row: usize, col: usize) -> Option<&u32> {
	heightmap.get(row)?.get(col)
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
