use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug)]
struct Line {
	p1: Point,
	p2: Point,
}

fn main() {
	let file_lines = read_lines("input")
		.expect("Can't read the input file");
	let hydro_lines: Vec<Line> = save_lines(file_lines);
	let mut hydro_points: HashMap<Point, u32> = HashMap::new();
	for line in hydro_lines {
		for point in calc_points_in_line(&line) {
			let count = hydro_points.entry(point).or_insert(0);
			*count += 1;
		}
	}
	println!("nb total hydro points: {}", hydro_points.len());
	let mut overlapping_count = 0;
	for (_key, value) in hydro_points {
		if value >= 2 {
			println!("overlapping point: {:?}", _key);
			overlapping_count += 1;
		}
	}
	println!("Number of overlapping hydrothermal points: {}", overlapping_count);
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn save_lines(file_lines: io::Lines<io::BufReader<File>>) -> Vec<Line> {
	let mut lines: Vec<Line> = Vec::new();

	for line in file_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() == false {
				let two_points: Vec<&str> = sgl_line.split(" -> ").collect();
				let curr_line = Line {
					p1: str_to_point(two_points.get(0).unwrap()),
					p2: str_to_point(two_points.get(1).unwrap()),
				};
				lines.push(curr_line);
			}
		}
	}
	lines
}

fn str_to_point(s: &str) -> Point {
	let p: Vec<i32> = s.split(',')
		.map(|nb| nb.parse().unwrap())
		.collect();
	Point {
		x: p.get(0).unwrap().clone(),
		y: p.get(1).unwrap().clone(),
	}
}

fn calc_points_in_line(line: &Line) -> Vec<Point> {
	let delta_x = line.p2.x - line.p1.x;
	let delta_y = line.p2.y - line.p1.y;
	let x_step: f64;
	let y_step: f64;
	let nb_steps;

	if delta_x.abs() >= delta_y.abs() {
		if delta_x > 0 { x_step = 1.; } else { x_step = -1.; }
		y_step = delta_y as f64 / delta_x.abs() as f64;
		nb_steps = delta_x.abs();
	}
	else {
		if delta_y > 0 { y_step = 1.; } else { y_step = -1.; }
		x_step = delta_x as f64 / delta_y.abs() as f64;
		nb_steps = delta_y.abs();
	}

	let mut points = Vec::new();

	println!("{:?}", line);
	println!("Delta x:{}    Delta y:{}", delta_x, delta_y);
	println!("x step:{}     y step:{}", x_step, y_step);
	
	for point_nb in 0..nb_steps + 1 {
		points.push(Point {
			x: line.p1.x + (point_nb as f64 * x_step) as i32,
			y: line.p1.y + (point_nb as f64 * y_step) as i32,
		});
	}
	println!("Points in the line");
	println!("{:?}", points);
	println!("");

	points
}
