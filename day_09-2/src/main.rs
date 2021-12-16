use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Point {
	h: u32,
	in_basin: bool,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let mut basins: Vec<u32> = Vec::new();
	let mut heightmap = init_heightmap(input_lines);
	for row in 0..heightmap.len() {
		for col in 0..heightmap.get(row).unwrap().len() {
			let curr_point = heightmap.get(row).unwrap().get(col).unwrap();
			let curr_point_h = curr_point.h;
			if is_low_point(&heightmap, &curr_point, row, col) {
				basins.push(set_basin(&mut heightmap, curr_point_h, row, col));
			}
		}
	}
	println!("Basin size: {:#?}", basins);
	basins.sort();
	basins.reverse();
	let mut three_bigger = 1;
	for basin in &basins[0..3] {
		three_bigger *= basin;
	}
	println!("Multiplication of 3 Bigger Basins: {}", three_bigger);
}

fn init_heightmap(input_lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<Point>> {
	let mut heightmap: Vec<Vec<Point>> = Vec::new();

	for line in input_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() == false {
				let points_in_line: Vec<Point> = sgl_line.chars()
					.map(|s| Point {
						h: s.to_string().parse().unwrap(),
						in_basin: false,
					})
					.collect();
				heightmap.push(points_in_line);
			}
		}
	}
	heightmap
}

fn is_low_point(heightmap: &Vec<Vec<Point>>,
	curr_point: &Point,
	row: usize,
	col: usize) -> bool {
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
		if nb.h <= curr_point.h { return false };
	}
	if let Some(nb) = right {
		if nb.h <= curr_point.h { return false };
	}
	if let Some(nb) = bottom {
		if nb.h <= curr_point.h { return false };
	}
	if let Some(nb) = left {
		if nb.h <= curr_point.h { return false };
	}
	true
}

fn get_point(heightmap: &Vec<Vec<Point>>, row: usize, col: usize) -> Option<&Point> {
	heightmap.get(row)?.get(col)
}

fn get_mut_point(heightmap: &mut Vec<Vec<Point>>, row: usize, col: usize) -> Option<&mut Point> {
	heightmap.get_mut(row)?.get_mut(col)
}

fn set_basin(heightmap: &mut Vec<Vec<Point>>,
	last_h: u32,
	row: usize,
	col: usize) -> u32 {
	let mut curr_basin_size = 0;
	
	if let Some(curr_point) = get_mut_point(heightmap, row, col) {
		if curr_point.h != 9
			&& curr_point.in_basin == false
			&& curr_point.h >= last_h {
			curr_point.in_basin = true;
			let curr_point_h = curr_point.h;
			curr_basin_size += 1;
			if row > 0 {
				curr_basin_size += set_basin(heightmap, curr_point_h, row - 1, col);
			}
			curr_basin_size += set_basin(heightmap, curr_point_h, row, col + 1);
			curr_basin_size += set_basin(heightmap, curr_point_h, row + 1, col);
			if col > 0 {
				curr_basin_size += set_basin(heightmap, curr_point_h, row, col - 1);
			}
		}
	}
	return curr_basin_size;
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
