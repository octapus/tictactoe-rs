mod board;
use crate::board::BoardError;
use crate::board::Board;

fn handle_move(board: &mut Board, player: i8) -> bool {
	if !(player == 1 || player == -1) {
		panic!("handle_move should only take 1 or -1");
	}
	'input_loop: loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).expect("Failed to read line");
		let mut index: [usize; 4] = [0; 4];
		for (i, x) in input.trim().chars().take(4).enumerate() {
			match x {
				'-' => index[i] = 0,
				'=' => index[i] = 1,
				'+' => index[i] = 2,
				_ => {
					println!("Input must match regex [-=+]{{4}}");
					continue 'input_loop;
				}
			}
		}
		match board.handle_move(player, index) {
			Err(BoardError::PlayerError) => panic!("bad player {}", player),
			Err(BoardError::MoveError) => panic!("bad move {:?}", index),
			Err(_) => {
				println!("That space is taken");
				continue 'input_loop;
			},
			Ok(term) => return term,
		}
	}
}

fn main() {
	let mut board = Board::new();
	let mut player: i8 = 1;
	loop {
		//print!("\x1B[2J\x1B[1;1H");
		board.print();
		if handle_move(&mut board, player) {
			board.print();
			break;
		}
		player = -player;
	}
}
