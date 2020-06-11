use super::player::Player;

pub enum GameResult {
    Win(Player),
    Tie,
    Incomplete,
}

pub struct Board {
    board: [[Option<Player>; 3]; 3],
    turn: Player,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            turn: Player::Player1,
        }
    }

    pub fn next_move(&mut self) {
        if let Player::Player1 = self.turn {
            self.turn = Player::Player2;
        } else {
            self.turn = Player::Player1;
        }
    }

    pub fn current_turn(&self) -> &Player {
        &self.turn
    }

    pub fn add_move(&mut self, player: Player, position: (usize, usize)) -> Result<(), &str> {
        if position.0 > self.board.len() || position.1 > self.board[0].len() {
            return Err("index out of bounds");
        }
        let current = &self.board[position.0][position.1];
        if let Some(_) = current {
            Err("player has already moved here")
        } else {
            self.board[position.0][position.1] = Some(player);
            Ok(())
        }
    }

    pub fn check_game_end(&self) -> GameResult {
        let row_size = self.board[0].len();
        for row in self.board.iter() {
            if let Some(p) = all_equal(row.iter()) {
                return GameResult::Win(p);
            }
        }
        for i in 0..row_size {
            let col = self.board.iter().map(|row| row.iter().nth(i)).flatten();
            if let Some(p) = all_equal(col) {
                return GameResult::Win(p);
            }
        }
        let diag = self
            .board
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().nth(i))
            .flatten();
        if let Some(p) = all_equal(diag) {
            return GameResult::Win(p);
        }
        let diag2 = self
            .board
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| row.iter().nth(i))
            .flatten();
        if let Some(p) = all_equal(diag2) {
            return GameResult::Win(p);
        }

        let complete =
            self.board
                .iter()
                .map(|row| row.iter())
                .flatten()
                .all(|square| match *square {
                    Some(_) => true,
                    None => false,
                });
        if complete {
            GameResult::Tie
        } else {
            GameResult::Incomplete
        }
    }
}

fn all_equal<'a>(iter: impl Iterator<Item = &'a Option<Player>>) -> Option<Player> {
    let mut iter = iter.peekable();
    let first = (**iter.peek().unwrap()).clone();
    while let Some(item) = iter.next() {
        if let Some(next) = iter.peek() {
            if item != *next {
                return None;
            }
        }
    }
    first
}

impl ToString for Board {
    fn to_string(&self) -> String {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|square| match square {
                        Some(Player::Player1) => "X",
                        Some(Player::Player2) => "O",
                        None => "_",
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
