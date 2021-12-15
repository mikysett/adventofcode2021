use std::fs;

fn main() {
	let mut lanternfish: Vec<u32> = fs::read_to_string("input").unwrap()
		.split(',')
		.map(|s| s.trim())
		.map(|s| s.parse().unwrap())
		.collect();
	for day in 1..81 {
		let mut newborn = 0;
		for fish in &mut lanternfish {
			if *fish > 0 {
				*fish -= 1;
			}
			else {
				*fish = 6;
				newborn += 1;
			}
		}
		for _i in 0..newborn {
			lanternfish.push(8);
		}
		println!("day {} - lanternfish: {}", day, lanternfish.len());
		
	}
}
