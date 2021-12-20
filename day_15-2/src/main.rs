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
	let last_pos = expand_map(&mut risk_map, &last_pos);

	// print_map(&risk_map, &last_pos);

	init_scores(&mut risk_map, &last_pos, Position {x: 0, y: 0}, 0);
	// println!("Last position: {:?}", last_pos);
	calculate_risks(&mut risk_map, &last_pos, Position {x: 1, y: 0}, 0);
	calculate_risks(&mut risk_map, &last_pos, Position {x: 0, y: 1}, 0);
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

fn expand_map(risk_map: &mut HashMap<Position, Point>, last_pos: &Position)
	-> Position {
	for col in 0..last_pos.x + 1 {
		for row in 0..last_pos.y + 1 {
			expand_point(risk_map,
				Position {
					x: col,
					y: row,
				}, &last_pos);
		}
	}
	Position {
		x: (last_pos.x + 1) * 5 - 1,
		y: (last_pos.y + 1) * 5 - 1,
	}
}

fn expand_point(risk_map: &mut HashMap<Position, Point>,
	point_pos: Position,
	last_pos: &Position) {
	let curr_point_risk = risk_map.get(&point_pos).unwrap().risk;
	for to_left in 0..5 {
		for to_bottom in 0..5 {
			let mut new_risk = (curr_point_risk + to_left + to_bottom) % 9;
			if new_risk == 0 {
				new_risk = 9;
			}
			risk_map.insert(
				Position {
					x: point_pos.x + to_left as usize * (last_pos.x as usize + 1),
					y: point_pos.y + to_bottom as usize * (last_pos.y as usize + 1),
				}, Point {
					risk: new_risk,
					best_score: 0,
				});
		}
	}
}

// fn init_scores(risk_map: &mut HashMap<Position, Point>, last_pos: &Position) {
// 	let mut column_risk = 0;
// 	for col in 0..last_pos.x + 1 {
// 		let mut curr_point = risk_map.get_mut(&Position {x: col, y: 0}).unwrap();
// 		column_risk += curr_point.risk;
// 		curr_point.best_score = column_risk;
// 		let mut point_risk = column_risk;
// 		for row in 1..last_pos.y + 1 {
// 			let mut curr_point = risk_map.get_mut(&Position {x: col, y: row}).unwrap();
// 			point_risk += curr_point.risk;
// 			curr_point.best_score = point_risk;
// 		}
// 	}
// }

fn init_scores(risk_map: &mut HashMap<Position, Point>,
	last_pos: &Position,
	curr_pos: Position,
	curr_score: u32) {
	if let Some(curr_point) = risk_map.get_mut(&curr_pos) {
		let curr_risk = curr_score + curr_point.risk;
		if curr_point.best_score != 0
			&& curr_risk >= curr_point.best_score {
			return ;
		}
		curr_point.best_score = curr_risk;
		if curr_pos == *last_pos {
			println!("BEST INIT SCORE  -- > {}", curr_point.best_score);
		}
		let last_point_score = risk_map.get(last_pos).unwrap().best_score;
		if curr_pos != *last_pos
			&& (last_point_score == 0 || curr_risk < last_point_score) {
			init_scores(risk_map, last_pos,
				Position {x: curr_pos.x + 1, y: curr_pos.y}, curr_risk);
			init_scores(risk_map, last_pos,
				Position {x: curr_pos.x, y: curr_pos.y + 1}, curr_risk);
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
			println!("BEST SCORE  -- > {}", curr_point.best_score);
		}
		if curr_pos != *last_pos
			&& curr_risk < risk_map.get(last_pos).unwrap().best_score {
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

// Utility function
fn print_map(risk_map: &HashMap<Position, Point>, last_pos: &Position) {
	for row in 0..last_pos.y + 1 {
		for col in 0..last_pos.x + 1 {
			print!("{}", risk_map.get(&Position {x: col, y: row}).unwrap().risk);
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
