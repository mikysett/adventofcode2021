use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BOARD_SIZE: u32 = 5;

type Board = Vec<Vec<BingoNb>>;

#[derive(Debug)]
pub struct BingoNb {
	value: u32,
	drawned: bool,
}

fn main() {
	let mut file_lines = read_lines("input")
		.expect("Can't read the input file");
	let nb_drawned: Vec<u32> = file_lines.nth(0).unwrap().unwrap() // Result is Option<io::Result<String>>
		.split(',')
		.map(|s| s.parse().unwrap())
		.collect();
	let mut boards: Vec<Board> = save_boards(file_lines);

	for drawned in nb_drawned {
		println!("Number drawned: {}", drawned);
		update_boards(&mut boards, drawned);
		while let Some(winner) = winner_index(&mut boards) {
			if boards.len() > 1 {
				println!("Winner removed");
				boards.remove(winner);
			}
			else {
				let winner_board = boards.get(0).unwrap();
				println!("LAST Winner board:");
				println!("{:#?}", winner_board);
				println!("Score: {}", calculate_score(winner_board, drawned));
				return
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

fn save_boards(file_lines: io::Lines<io::BufReader<File>>) -> Vec<Board> {
	let mut boards: Vec<Board> = Vec::new();
	let mut sgl_board: Board = Vec::new();

	for line in file_lines {
		if let Ok(sgl_line) = line {
			if sgl_line.is_empty() {
				if !sgl_board.is_empty() {
					boards.push(sgl_board);
					sgl_board = Vec::new();
				}
				continue;
			}
			sgl_board.push(sgl_line.split(' ')
				.map(|s| s.trim())
				.filter(|s| !s.is_empty())
				.map(|s| BingoNb {
						value: s.parse::<u32>().unwrap(),
						drawned: false
					})
				.collect());
		}
	}
	if !sgl_board.is_empty() {
		boards.push(sgl_board)
	}
	boards
}

fn update_boards(boards: &mut Vec<Board>, nb_drawned: u32) {
	for board in boards {
		'board_check: for line in board {
			for nb in line {
				if nb.value == nb_drawned {
					nb.drawned = true;
					break 'board_check;
				}
			}
		}
	}
}

fn winner_index(boards: &mut Vec<Board>) -> Option<usize> {
	let board_len = boards.len();

	for i in 0..board_len as usize {
		if let Some(winner) = check_rows(&boards.get(i).unwrap()) {
			return Some(i);
		}
		else if let Some(winner) = check_columns(&boards.get(i).unwrap()) {
			return Some(i);
		}
	}
	None
}

fn check_rows(board: &Board) -> Option<&Board> {
	'line_check: for line in board {
		for nb in line {
			if nb.drawned == false {
				continue 'line_check;
			}
		}
		return Some(board)
	}
	None
}

fn check_columns(board: &Board) -> Option<&Board> {
	'col_check: for col in 0..BOARD_SIZE as usize {
		for row in board {
			if row.get(col).unwrap().drawned == false {
				continue 'col_check;
			}
		}
		return Some(board)
	}
	None
}

fn calculate_score(board: &Board, last_nb: u32) -> u32 {
	let mut score = 0;

	for line in board {
		for nb in line {
			if nb.drawned == false {
				score += nb.value;
			}
		}
	}
	score * last_nb
}