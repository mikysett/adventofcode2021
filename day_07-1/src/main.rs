use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Boundaries {
	min: u32,
	max: u32,
}

fn main() {
	let crabs: Vec<u32> = fs::read_to_string("input").unwrap()
		.split(',')
		.map(|s| s.trim())
		.map(|s| s.parse().unwrap())
		.collect();

	let boundaries = calculate_boundaries(&crabs);
	let mut possible_positions = set_positions(&crabs, &boundaries);
	let mut best_pos = match possible_positions.entry(boundaries.min) {
		Entry::Occupied(pos) => pos.get().clone(),
		_ => panic!("Entry in boundaries not found")
	};
	// println!("{:#?}", possible_positions);
	for (_key, pos) in possible_positions {
		if pos < best_pos {
			best_pos = pos.clone();
		}
	}
	println!("Best position (fuel cost): {}", best_pos);
}

fn calculate_boundaries(crabs: &Vec<u32>) -> Boundaries {
	let mut min = crabs.get(0).unwrap().clone();
	let mut max = min.clone();
	for crab in crabs {
		if *crab < min {
			min = *crab;
		}
		else if *crab > max {
			max = *crab;
		}
	}
	Boundaries {
		min,
		max,
	}
}

fn set_positions(crabs: &Vec<u32>, boundaries: &Boundaries) -> HashMap<u32, u32> {
	let mut possible_pos = HashMap::new();
	for pos in boundaries.min..boundaries.max + 1 {
		let mut fuel = 0;
		for crab in crabs {
			let distance;
			if *crab >= pos {
				distance = crab - pos;
			}
			else {
				distance = pos - crab;
			}
			fuel += distance;
		}
		possible_pos.entry(pos).or_insert(fuel);
	}
	possible_pos
}