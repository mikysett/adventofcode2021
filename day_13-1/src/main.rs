use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
	x: u32,
	y: u32,
}

#[derive(Debug)]
struct Fold {
	fold_type: char,
	position: u32,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let (mut points, folds) = init_inputs(input_lines);
	
	println!("Points: {:#?}", points);
	println!("Folds: {:#?}", folds);

	perform_folds(&mut points, &folds);
	points.sort_unstable();
	points.dedup();
	
	println!("Number of points: {}", points.len());
}

fn init_inputs(input_lines: io::Lines<io::BufReader<File>>) -> (Vec<Point>, Vec<Fold>) {
	let mut save_points = true;
	let mut points: Vec<Point> = Vec::new();
	let mut folds: Vec<Fold> = Vec::new();
	for line in input_lines.flatten() {
		if line.is_empty() {
			save_points = false;
		}
		else if save_points {
			let point_values: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
			points.push(Point {
				x: *point_values.get(0).unwrap(),
				y: *point_values.get(1).unwrap(),
			});
		}
		else {
			let curr_fold: Vec<&str> = line.split(' ')
				.filter(|s| {
					let first_c = s.chars().next().unwrap();
					first_c == 'x' || first_c == 'y'
				}).collect();
			let curr_fold: Vec<&str> = curr_fold.get(0).unwrap().split('=').collect();
			let curr_fold = Fold {
				fold_type: curr_fold.get(0).unwrap().chars().next().unwrap(),
				position: curr_fold.get(1).unwrap().parse().unwrap(),
			};
			folds.push(curr_fold);
		}
	}
	(points, folds)
}

fn perform_folds(points: &mut Vec<Point>, folds: &[Fold]) {
	let mut fold_nb = 0;
	for fold in folds {
		if fold_nb > 0 { break; } // To fold just once!
		if fold.fold_type == 'x' {
			fold_on_vertical_line(points, fold.position);
		}
		else {
			fold_on_horizontal_line(points, fold.position);
		}
		fold_nb += 1;
	}
}

fn fold_on_vertical_line(points: &mut Vec<Point>, fold_pos: u32) {
	for point in points {
		if point.x > fold_pos {
			point.x = fold_pos - (point.x - fold_pos);
		}
	}
}

fn fold_on_horizontal_line(points: &mut Vec<Point>, fold_pos: u32) {
	for point in points {
		if point.y > fold_pos {
			point.y = fold_pos - (point.y - fold_pos);
		}
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
