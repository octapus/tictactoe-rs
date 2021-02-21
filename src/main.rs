type Board = [[[[i8; 3]; 3]; 3]; 3];

fn print_board(board: &Board) {
	for w in (0..3).rev() {
		for y in (0..3).rev() {
			for z in 0..3 {
				for x in 0..3 {
					match board[w][z][y][x] {
						-2 => print!("O"),
						-1 => print!("o"),
						0 => print!("."),
						1 => print!("x"),
						2 => print!("X"),
						_ => print!("?"),
					}
					print!(" ");
				}
				print!(" ");
			}
			println!();
		}
		println!();
	}
}

fn handle_move(board: &mut Board, player: i8) {
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
		if board[index[0]][index[1]][index[2]][index[3]] != 0 {
			println!("That space is already taken");
			continue 'input_loop;
		}
		board[index[0]][index[1]][index[2]][index[3]] = player;
		break;
	}
}

fn main() {
	let mut board: Board = [[[[0; 3]; 3]; 3]; 3];
	let mut player: i8 = 1;
	loop {
		//print!("\x1B[2J\x1B[1;1H");
		print_board(&board);
		handle_move(&mut board, player);
		player = -player;
	}
}
