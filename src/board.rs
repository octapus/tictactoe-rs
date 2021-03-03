pub enum BoardError {
	PlayerError,
	MoveError,
	SpaceTaken,
}

pub struct Board {
	board: [[[[i8; 3]; 3]; 3]; 3],
}

impl Board {
	pub fn new() -> Board {
		Board {
			board: [[[[0; 3]; 3]; 3]; 3],
		}
	}

	pub fn print(&self) {
		for w in (0..3).rev() {
			for y in (0..3).rev() {
				for z in 0..3 {
					for x in 0..3 {
						match self.board[w][z][y][x] {
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

	pub fn handle_move(&mut self, player: i8, pos: [usize; 4]) -> Result<(), BoardError> {
		if !(player == 1 || player == -1) {
			return Err(BoardError::PlayerError);
		}
		if pos.iter().any(|x| *x > 2usize) {
			return Err(BoardError::MoveError);
		}
		if self.board[pos[0]][pos[1]][pos[2]][pos[3]] != 0 {
			return Err(BoardError::SpaceTaken);
		}
		self.board[pos[0]][pos[1]][pos[2]][pos[3]] = player;
		return Ok(());
	}
}

