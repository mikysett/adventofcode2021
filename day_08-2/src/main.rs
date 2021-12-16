use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Screen {
	input: Vec<String>,
	output: Vec<String>,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	let mut total_nb = 0;
	
	for line in input_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() == false {
				let line_sides: Vec<&str> = sgl_line.split(" | ").collect();
				let curr_screen = get_curr_screen(line_sides);
				let wires = decode_wires(&curr_screen);
				let decoded_nb = calc_nb(&wires, &curr_screen.output);
				// println!("decoded_nb: {}", decoded_nb);
				total_nb += decoded_nb;
			}
		}
	}
	println!("total nb: {}", total_nb);
}

fn get_curr_screen(line_sides: Vec<&str>) -> Screen {
	Screen {
		input: line_sides.get(0).unwrap()
			.split(' ')
			.map(|s| s.to_string())
			.collect(),
		output: line_sides.get(1).unwrap()
			.split(' ')
			.map(|s| s.to_string())
			.collect(),
	}
}

fn decode_wires(screen: &Screen) -> HashMap<u32, String> {
	let mut wires: HashMap<u32, String> = HashMap::new();

	set_uniques(&screen, &mut wires);
	set_nine(&screen, &mut wires);
	set_zero(&screen, &mut wires);
	set_six(&screen, &mut wires);
	set_three(&screen, &mut wires);
	set_five(&screen, &mut wires);
	set_two(&screen, &mut wires);
	// println!("Screen: {:#?}", screen);
	// println!("wires: {:?}", wires);
	// println!("");

	wires
}

fn calc_nb(wires: &HashMap<u32, String>, screen_output: &Vec<String>) -> u32 {
	let mut final_nb = 0;

	for nb in screen_output {
		for (key, value) in wires {
			if nb.len() == value.len()
				&& nb_differences(nb, value) == 0 {
				final_nb = (final_nb * 10) + key;
			}
		}
	}
	final_nb
}

fn set_uniques(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 2 {
			wires.entry(1).or_insert(code.clone());
		}
		else if code.len() == 3 {
			wires.entry(7).or_insert(code.clone());
		}
		else if code.len() == 4 {
			wires.entry(4).or_insert(code.clone());
		}
		else if code.len() == 7 {
			wires.entry(8).or_insert(code.clone());
		}
	}
}

fn set_nine(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 6 && !already_wired(&wires, &code) {
			if nb_differences(
				wires.entry(4).or_insert(String::new()),
				code)
				== 0 {
					wires.entry(9).or_insert(code.clone());
					break;
				}
		}
	}
}

fn set_zero(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 6 && !already_wired(&wires, &code) {
			if nb_differences(
				wires.entry(1).or_insert(String::new()),
				code)
				== 0 {
				wires.entry(0).or_insert(code.clone());
				break;
			}
		}
	}
}

fn set_six(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 6 && !already_wired(&wires, &code) {
			wires.entry(6).or_insert(code.clone());
			break;
		}
	}
}

fn set_three(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 5 && !already_wired(&wires, &code) {
			if nb_differences(
					wires.entry(1).or_insert(String::new()),
					code)
				== 0 {
				wires.entry(3).or_insert(code.clone());
				break;
			}
		}
	}
}

fn set_five(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 5 && !already_wired(&wires, &code) {
			if nb_differences(
				code,	
				wires.entry(9).or_insert(String::new()))
				== 0 {
				wires.entry(5).or_insert(code.clone());
				break;
			}
		}
	}
}

fn set_two(screen: &Screen, wires: &mut HashMap<u32, String>) {
	for code in &screen.input {
		if code.len() == 5 && !already_wired(&wires, &code) {
			wires.entry(2).or_insert(code.clone());
			break;
		}
	}
}

fn nb_differences(s1: &String, s2: &String) -> u32 {
	let mut nb_diff = 0;

	for c in s1.chars() {
		match s2.find(c) {
			Some(_) => continue,
			None => nb_diff += 1,
		};
	}
	nb_diff
}

fn already_wired(wires: &HashMap<u32, String>, code: &String) -> bool {
	for (_key, nb) in wires {
		if nb == code {
			return true;
		}
	}
	false
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
