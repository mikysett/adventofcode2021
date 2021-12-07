use std::fs;

fn main() {
    let input = fs::read_to_string("input")
		.expect("Can't open input file");
	let distances = input.split('\n');
	
	let mut old_distance = 0;
	let mut nb_increments = 0;
	for distance in distances {
		let distance: u32 = match distance.trim().parse() {
			Ok(number) => number,
			Err(_) => {
				continue;
			}
		};
		if distance > old_distance {
			nb_increments += 1;
		}
		old_distance = distance;
	}
	nb_increments -= 1;
	println!("Number of increments: {}", nb_increments);
}
