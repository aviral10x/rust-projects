use std::io;
use std::fmt;

const BOARD_SIZE: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Board {
    grid: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn print(&self) {
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some(player) => print!(" {} ", player),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    fn is_full(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn place_token(&mut self, x: usize, y: usize, player: Player) -> Result<(), String> {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            return Err(String::from("Position is out of the board!"));
        }
        if self.grid[x][y].is_some() {
            return Err(String::from("Position is already taken!"));
        }
        self.grid[x][y] = Some(player);
        Ok(())
    }

    fn check_winner(&self) -> Option<Player> {
        // Check rows, columns and diagonals
        let mut main_diag = [None; BOARD_SIZE];
        let mut anti_diag = [None; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            // Check rows and columns
            if self.grid[i].iter().all(|&cell| cell == Some(Player::X))
                || self.grid.iter().all(|row| row[i] == Some(Player::X))
            {
                return Some(Player::X);
            }
            if self.grid[i].iter().all(|&cell| cell == Some(Player::O))
                || self.grid.iter().all(|row| row[i] == Some(Player::O))
            {
                return Some(Player::O);
            }

            // Check diagonals
            main_diag[i] = self.grid[i][i];
            anti_diag[i] = self.grid[i][BOARD_SIZE - 1 - i];
        }
        if main_diag.iter().all(|&cell| cell == Some(Player::X))
            || anti_diag.iter().all(|&cell| cell == Some(Player::X))
        {
            return Some(Player::X);
        }
        if main_diag.iter().all(|&cell| cell == Some(Player::O))
            || anti_diag.iter().all(|&cell| cell == Some(Player::O))
        {
            return Some(Player::O);
        }
        None
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;
    loop {
        println!("Current board:");
        board.print();

        let (x, y) = get_user_input(&current_player);
        match board.place_token(x, y, current_player) {
            Ok(_) => {
                if let Some(winner) = board.check_winner() {
                    println!("Player {} wins!", winner);
                    board.print();
                    break;
                } else if board.is_full() {
                    println!("It's a draw!");
                    board.print();
                    break;
                }
                current_player = if current_player == Player::X {
                    Player::O
                } else {
                    Player::X
                };
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn get_user_input(current_player: &Player) -> (usize, usize) {
    loop {
        println!("Player {}, enter your move as 'row col' (e.g., '1 2'):", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Please enter two numbers.");
            continue;
        }

        let x = parts[0].parse::<usize>();
        let y = parts[1].parse::<usize>();

        match (x, y) {
            (Ok(x), Ok(y)) if x < BOARD_SIZE && y < BOARD_SIZE => return (x, y),
            _ => println!("Invalid input. Please enter numbers between 0 and {}.", BOARD_SIZE - 1),
        }
    }
}
