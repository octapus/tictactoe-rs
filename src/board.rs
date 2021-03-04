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

	pub fn handle_move(&mut self, player: i8, pos: [usize; 4]) -> Result<bool, BoardError> {
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
		return Ok(self.handle_wins(pos, player));
	}


	/**
	* Checks if there is a win in a specific direction.
	* If the direction is off the board, it returns false.
	* Otherwise it will reverse directions if necessary when checking the second in a row.
	*/
	fn check_win(&self, x: usize, y: usize, z: usize, w: usize, dx: i8, dy: i8, dz: i8, dw: i8, player: i8) -> bool {
		let x1 = x as i8 +dx;
		let y1 = y as i8 +dy;
		let z1 = z as i8 +dz;
		let w1 = w as i8 +dw;

		if x1 > 2 || y1 > 2 || z1 > 2 || w1 > 2 {
			return false;
		}
		if x1 < 0 || y1 < 0 || z1 < 0 || w1 < 0 {
			return false;
		}
		if self.board[x1 as usize][y1 as usize][z1 as usize][w1 as usize] != player {
			return false;
		}

		let mut x2 = x as i8 -dx;
		let mut y2 = y as i8 -dy;
		let mut z2 = z as i8 -dz;
		let mut w2 = w as i8 -dw;
		if x2 < 0 || x2 > 2 || y2 < 0 || y2 > 2 || z2 < 0 || z2 > 2 || w2 < 0 || w2 > 2  {
			x2 = x as i8 +2*dx;
			y2 = y as i8 +2*dy;
			z2 = z as i8 +2*dz;
			w2 = w as i8 +2*dw;
		}
		if x2 < 0 || x2 > 2 || y2 < 0 || y2 > 2 || z2 < 0 || z2 > 2 || w2 < 0 || w2 > 2  {
			return false;
		}
		if self.board[x2 as usize][y2 as usize][z2 as usize][w2 as usize] != player {
			return false;
		}

		return true;
	}

	/**
	* Returns a list of the directions of wins (3 in a rows) for a specific move
	*/
	fn get_wins(&self, x: usize, y: usize, z: usize, w: usize, player: i8) -> Vec<[i8; 4]> {
		let mut directions: Vec<[i8; 4]> = Vec::new();
		for dw in -1..2 {
			for dz in -1..2 {
				for dy in -1..2 {
					for dx in -1..2 {
						// Skip if all directions are 0
						if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
							continue;
						}

						// Check if direction contains 3 in a row
						if self.check_win(x, y, z, w, dx, dy, dz, dw, player) {
							// Confirm it is not a duplicate of another win (in the opposite direction)
							let mut duplicate = false;
							for win in &directions {
								if dx == -win[0] && dy == -win[1] && dz == -win[2] && dw == -win[3]  {
									duplicate = true;
									break;
								}
							}

							if !duplicate {
								directions.push([dx, dy, dz, dw]);
							}
						}
					}
				}
			}
		}
		return directions;
	}

	fn handle_wins(&mut self, pos: [usize; 4], player: i8) -> bool {
		let directions: Vec<[i8; 4]> = self.get_wins(pos[0], pos[1], pos[2], pos[3], player);
		// mark wins
		if directions.len() >= 2 {
			for win in directions {
				let x1 = (pos[0] as i8 +win[0]) as usize;
				let y1 = (pos[1] as i8 +win[1]) as usize;
				let z1 = (pos[2] as i8 +win[2]) as usize;
				let w1 = (pos[3] as i8 +win[3]) as usize;
				self.board[x1][y1][z1][w1] *= 2;

				let mut x2 = pos[0] as i8 -win[0];
				let mut y2 = pos[1] as i8 -win[1];
				let mut z2 = pos[2] as i8 -win[2];
				let mut w2 = pos[3] as i8 -win[3];
				if x2 < 0 || x2 > 2 || y2 < 0 || y2 > 2 || z2 < 0 || z2 > 2 || w2 < 0 || w2 > 2  {
					x2 = pos[0] as i8 +2*win[0];
					y2 = pos[1] as i8 +2*win[1];
					z2 = pos[2] as i8 +2*win[2];
					w2 = pos[3] as i8 +2*win[3];
				}
				self.board[x2 as usize][y2 as usize][z2 as usize][w2 as usize] *= 2;
			}
			self.board[pos[0]][pos[1]][pos[2]][pos[3]] *= 2;
			return true;
		}
		return false;
	}
}

