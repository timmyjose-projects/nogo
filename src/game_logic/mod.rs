
/// This module contains all the game-related logic

pub mod validation;

use std::fmt::{Formatter, Result, Display};
use error_handling as eh;


/// Some game constants

pub const MIN_BOARD_DIMENSION: i32 = 4;
pub const MAX_BOARD_DIMENSION: i32 = 1000;

/// Game related data structures
#[derive(Debug)]
pub enum PlayerType {
    HUMAN,
    COMPUTER,
}

#[derive(Debug)]
struct NogoBoard {
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
}

#[derive(Debug)]
struct NogoBoardState {
    players: (NogoPlayer, NogoPlayer),
}

impl NogoBoardState {
    fn new(p1: PlayerType, p2: PlayerType) -> Self {
        NogoBoardState { players: (NogoPlayer::new(0, p1), NogoPlayer::new(1, p2)) }
    }
}

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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


///
/// Start a fresh game
///
pub fn start_new_game<'a>(p1: &'a str, p2: &'a str, height: &'a str, width: &'a str) {
    /// check if the arguments are correct
    match validation::validate_new_game_parameters(p1, p2, height, width) {
        Ok((p1, p2, h, w)) => {
            let mut board = create_board(p1, p2, h, w);

            loop {
                display_board(&board);
                {
                    update_board(0, &mut board);
                }

                {
                    update_board(1, &mut board);
                }

                if let Some(player) = check_winner(&board) {
                    println!("Player {} wins", player.id);
                    break;
                }
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

fn create_board(p1: PlayerType, p2: PlayerType, h: i32, w: i32) -> NogoBoard {
    NogoBoard::new(p1, p2, h, w)
}

fn display_board(board: &NogoBoard) {}

fn update_board(player: i32, board: &mut NogoBoard) {}

fn check_winner(board: &NogoBoard) -> Option<NogoPlayer> {
    None
}
