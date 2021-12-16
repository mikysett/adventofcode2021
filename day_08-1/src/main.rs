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

	let screens = save_screens(input_lines);
	// println!("{:#?}", screens);
	let unique_nb = count_unique_nb(&screens);
	println!("Count: {}", unique_nb);
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn save_screens(file_lines: io::Lines<io::BufReader<File>>) -> Vec<Screen> {
	let mut screens: Vec<Screen> = Vec::new();

	for line in file_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() == false {
				let line_sides: Vec<&str> = sgl_line.split(" | ").collect();
				let curr_screen = Screen {
					input: line_sides.get(0).unwrap()
						.split(' ')
						.map(|s| s.to_string())
						.collect(),
					output: line_sides.get(1).unwrap()
						.split(' ')
						.map(|s| s.to_string())
						.collect(),
				};
				screens.push(curr_screen);
			}
		}
	}
	screens
}

fn count_unique_nb(screens: &Vec<Screen>) -> u32 {
	let mut count = 0;
	
	for screen in screens {
		for nb in &screen.output {
			let nb_len = nb.len();
			if nb_len == 2 || nb_len == 3 || nb_len == 4 || nb_len == 7 {
				count += 1;
			}
		}
	}
	count
}