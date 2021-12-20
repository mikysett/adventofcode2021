use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Point {
	risk: u32,
	best_score: u32,
}

#[derive(Debug)]
struct Size {
	w: usize,
	h: usize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
	x: usize,
	y: usize,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let (mut risk_map, last_pos) = init_map(input_lines);
	init_scores(&mut risk_map, &last_pos);
	println!("Last position: {:?}", last_pos);
	calculate_risks(&mut risk_map, &last_pos, Position {x: 1, y: 0}, 0);
	calculate_risks(&mut risk_map, &last_pos, Position {x: 0, y: 1}, 0);
	// println!("Risk map after calc:\n{:#?}\n", risk_map);
	println!("Best score for {:?}: {}",
		last_pos,
		risk_map.get(&last_pos).unwrap().best_score);
}

fn init_map(input_lines: io::Lines<io::BufReader<File>>)
	-> (HashMap<Position, Point>, Position) {
	let mut map: HashMap<Position, Point> = HashMap::new();
	let mut last_position = Position {
		x: 0,
		y: 0,
	};

	for (row, line) in input_lines.flatten().enumerate() {
		if line.is_empty() {
			continue;
		}
		last_position.y += 1;
		if last_position.x == 0 {
			last_position.x = line.len() - 1;
		}
		for (column, c) in line.chars().enumerate() {
			let curr_risk = c.to_string().parse().unwrap();
			map.insert(
				Position {x: column, y: row},
				Point {
					risk: curr_risk,
					best_score: 0,		
			});
		}
	}
	map.get_mut(&Position {x: 0, y: 0}).unwrap().best_score = 0;
	last_position.y -= 1;
	(map, last_position)
}

fn init_scores(risk_map: &mut HashMap<Position, Point>, last_pos: &Position) {
	let mut column_risk = 0;
	for col in 0..last_pos.x + 1 {
		let mut curr_point = risk_map.get_mut(&Position {x: col, y: 0}).unwrap();
		column_risk += curr_point.risk;
		curr_point.best_score = column_risk;
		let mut point_risk = column_risk;
		for row in 1..last_pos.y + 1 {
			let mut curr_point = risk_map.get_mut(&Position {x: col, y: row}).unwrap();
			point_risk += curr_point.risk;
			curr_point.best_score = point_risk;
		}
	}
}

fn calculate_risks(risk_map: &mut HashMap<Position, Point>,
	last_pos: &Position,
	curr_pos: Position,
	curr_score: u32) {
	if let Some(curr_point) = risk_map.get_mut(&curr_pos) {
		let curr_risk = curr_score + curr_point.risk;
		if curr_risk >= curr_point.best_score {
			return ;
		}
		// println!("{:?}", curr_pos);
		curr_point.best_score = curr_risk;
		if curr_pos == *last_pos {
			println!("-------------- >");
			println!("BEST SCORE  -- > {}", curr_point.best_score);
			println!("-------------- >");
		}
		if curr_pos != *last_pos
			&& curr_risk < risk_map.get_mut(last_pos).unwrap().best_score {
			calculate_risks(risk_map, last_pos,
				Position {x: curr_pos.x + 1, y: curr_pos.y}, curr_risk);
			calculate_risks(risk_map, last_pos,
				Position {x: curr_pos.x, y: curr_pos.y + 1}, curr_risk);
			if curr_pos.x > 0 {
				calculate_risks(risk_map, last_pos,
					Position {x: curr_pos.x - 1, y: curr_pos.y}, curr_risk);
			}
			if curr_pos.y > 0 {
				calculate_risks(risk_map, last_pos,
					Position {x: curr_pos.x, y: curr_pos.y - 1}, curr_risk);
			}
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
