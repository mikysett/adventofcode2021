use std::fs;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
enum Cmd {
	Forward(u32),
	Down(u32),
	Up(u32),
	Error,
}

struct Position {
	horizontal: u32,
	depth: u32,
}

fn main() {
	let input = fs::read_to_string("input")
		.expect("Can't open input file");
	let instructions = input.split('\n');

	let mut pos = Position {
		horizontal: 0,
		depth: 0,
	};
	for instr in instructions {
		let curr_instr = parse_instr(&instr);
		update_position(&curr_instr, &mut pos);
	}
	println!("Position:");
	println!("Horizontal : {}", pos.horizontal);
	println!("Depth      : {}", pos.depth);
	println!("Hor * Depth: {}", pos.horizontal * pos.depth);
}

fn parse_instr(instr: &str) -> Cmd {
	let instr: Vec<&str> = instr.split(' ').collect();
	if instr.len() != 2 {
		Cmd::Error
	}
	else if instr[0] == "forward" {
		match instr[1].trim().parse() {
			Ok(number) => Cmd::Forward(number),
			Err(_) => Cmd::Error
		}
	}
	else if instr[0] == "down" {
		match instr[1].trim().parse() {
			Ok(number) => Cmd::Down(number),
			Err(_) => Cmd::Error
		}
	}
	else if instr[0] == "up" {
		match instr[1].trim().parse() {
			Ok(number) => Cmd::Up(number),
			Err(_) => Cmd::Error
		}
	}
	else {
		Cmd::Error
	}
}

fn update_position(instr: &Cmd, position: &mut Position) {
	match instr {
		Cmd::Forward(nb) => position.horizontal += nb,
		Cmd::Down(nb) => position.depth += nb,
		Cmd::Up(nb) => position.depth -= nb,
		Cmd::Error => return,
	}
}