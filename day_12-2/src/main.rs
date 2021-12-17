use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Cave {
	name: String,
	size: CaveSize,
	connections: Vec<String>,
}

#[derive(Debug, Eq, PartialEq)]
enum CaveSize {
	Big,
	Small,
}

#[derive(Debug)]
struct CavesPath {
	caves: Vec<String>,
	visited_small_twice: bool,
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");
	
	let caves = init_cavemap(input_lines);
	// println!("cavemap: {:#?}", caves);

	let mut paths: Vec<CavesPath> = Vec::new();
	set_paths(&mut paths,
		&caves,
		caves.get("start").unwrap(),
		&CavesPath{ caves: Vec::new(), visited_small_twice: false });
	println!("paths: {:#?}", paths);
	println!("Number of possible paths: {}", paths.len());
}

fn init_cavemap(input_lines: io::Lines<io::BufReader<File>>) -> HashMap<String, Cave> {
	let mut caves: HashMap<String, Cave> = HashMap::new();

	for line in input_lines.flatten() {
		if !line.is_empty() {
			let caves_line: Vec<&str> = line.split('-').collect();
			for i in 0..caves_line.len() {
				let cave_name = get_name(&caves_line, i);
				let neighbor_name =
					if i == 0 { get_name(&caves_line, i + 1) }
					else { get_name(&caves_line, i - 1) };
				let curr_cave = caves.entry(cave_name.clone()).or_insert(Cave {
					name: cave_name.clone(),
					size: get_cave_size(&cave_name),
					connections: Vec::new()
				});
				curr_cave.connections.push(neighbor_name);
			}
		}
	}
	caves
}

fn get_name(caves_line: &[&str], index: usize) -> String {
	caves_line.get(index).unwrap().to_string()
}

fn get_cave_size(name: &str) -> CaveSize {
	if name.chars().next().unwrap().is_lowercase() {
		CaveSize::Small
	} else {
		CaveSize::Big
	}
}

fn set_paths(paths: &mut Vec<CavesPath>,
	caves: &HashMap<String, Cave>,
	curr_cave: &Cave,
	curr_path: &CavesPath) {
	let mut curr_path = CavesPath {
		caves: curr_path.caves.clone(),
		visited_small_twice: curr_path.visited_small_twice,
	};
	if curr_cave.size == CaveSize::Small
		&& curr_path.caves.contains(&curr_cave.name)
		&& (curr_cave.name == "start" || curr_path.visited_small_twice)
	{
		// println!("REMOVED: {:?} + {}", curr_path.caves, curr_cave.name);
	}
	else {
		if curr_cave.size == CaveSize::Small
			&& curr_path.caves.contains(&curr_cave.name) {
			curr_path.visited_small_twice = true;
		}
		curr_path.caves.push(curr_cave.name.clone());
		if curr_cave.name == "end" {
			paths.push(curr_path);
		}
		else {
			for next_cave in &curr_cave.connections {
				set_paths(paths, caves, caves.get(next_cave).unwrap(), &curr_path);
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
