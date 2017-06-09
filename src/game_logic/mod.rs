/// This module contains all the game-related logic

use error_handling as eh;
use io;

/// Some game constants

pub const MIN_BOARD_DIMENSION: i32 = 4;
pub const MAX_BOARD_DIMENSION: i32 = 1000;

/// Game related data structures
#[derive(Debug)]
pub enum PlayerType {
    HUMAN,
    COMPUTER,
}

/// the overall board -it holds state, but does
/// not really do any processng on its own
#[derive(Debug)]
pub struct NogoBoard {
    height: i32,
    width: i32,
    state: NogoBoardState,
}

impl NogoBoard {
    fn new(p1: PlayerType, p2: PlayerType, h: i32, w: i32) -> Self {
        NogoBoard {
            height: h,
            width: w,
            state: NogoBoardState::new(p1, p2),
        }
    }

    fn player(&mut self, id: u8) -> Option<&mut NogoPlayer> {
        match self.state.players {
            (ref mut p1, ref mut p2) => {
                if p1.id == id {
                    Some(p1)
                } else if p2.id == id {
                    Some(p2)
                } else {
                    None
                }
            }
        }
    }

    // get the positions occupied by all strings of both
    // players as (row, column) pairs
    pub fn coords(&self) -> Vec<(i32, i32)> {
        self.state
            .players
            .0
            .strings
            .iter()
            .chain(self.state.players.1.strings.iter())
            .map(|ref t| (t.x, t.y))
            .collect::<Vec<_>>()
    }
}

/// this holds the game state by holding
/// references to the current players of
/// the game
#[derive(Debug)]
struct NogoBoardState {
    players: (NogoPlayer, NogoPlayer),
}

impl NogoBoardState {
    fn new(p1: PlayerType, p2: PlayerType) -> Self {
        NogoBoardState { players: (NogoPlayer::new(0, p1), NogoPlayer::new(1, p2)) }
    }
}

/// this represents a player in the game.
/// each player holds the vector of "strings"
/// that he/she owns
#[derive(Debug)]
struct NogoPlayer {
    id: u8,
    strings: Vec<Point>,
    human: bool,
}

impl NogoPlayer {
    fn new(id: i32, typ: PlayerType) -> Self {
        NogoPlayer {
            id: id as u8,
            strings: Vec::new(),
            human: match typ {
                PlayerType::COMPUTER => false,
                _ => true,
            },
        }
    }
}

/// represents a point in the board
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}


///
/// Start a fresh game
///
pub fn start_new_game<'a>(p1: &'a str, p2: &'a str, height: &'a str, width: &'a str) {
    /// check if the arguments are correct
    match eh::validation::validate_new_game_parameters(p1, p2, height, width) {
        Ok((p1, p2, h, w)) => {
            let mut board = create_board(p1, p2, h, w);

            loop {
                display_board(&board);

                {
                    update_board(0, &mut board);
                }

                check_winner(&board);

                {
                    update_board(1, &mut board);
                }

                check_winner(&board);
            }
        }

        Err(e) => eh::exit_with_error(e),
    }
}



///
/// Load a saved game
///
pub fn continue_saved_game<'a>(save_file: &'a str) -> eh::Result<'a, ()> {
    Ok(())
}


///
/// Game logic related functions
///

/// create a fresh board with the given dimensions
fn create_board(p1: PlayerType, p2: PlayerType, h: i32, w: i32) -> NogoBoard {
    NogoBoard::new(p1, p2, h, w)
}

/// display the current state of the board
fn display_board(board: &NogoBoard) {
    print_head(board.width);

    // collect the string coordinates for both players
    // in one go so that a single pass will be sufficient
    // to display the board
    print_rows(board);

    print_tail(board.width);
}

fn print_head(n: i32) {
    print!("/");

    for _ in 0..n {
        print!("-");
    }
    println!("\\");
}

fn print_rows(board: &NogoBoard) {
    let coords = board.coords();

    for i in 0..board.height {
        print!("|");

        for j in 0..board.width {
            if coords.iter()
                .find(|&&t| t == (i, j)) == Some(&(i, j)) {
                print!("-");
            } else {
                print!(".");
            }
        }
        println!("|");
    }

}

fn print_tail(n: i32) {
    print!("\\");

    for _ in 0..n {
        print!("-");
    }
    println!("/\n");
}

/// update the board state with a player move
fn update_board(player: i32, board: &mut NogoBoard) {
    let player_name = match player {
        0 => '0',
        1 => 'X',
        _ => ' ',
    };

    let (r, c) = io::get_player_move(&board, player_name);

    if let Some(mut player) = board.player(player as u8) {
        player.strings.push(Point { x: r, y: c });
    }
}

/// check if a winner can be established
/// to do this, the basic rules of the game
/// must be checked to see if anyt string
/// of either player has been captured
fn check_winner(board: &NogoBoard) {}
