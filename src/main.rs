extern crate nogo;

use std::str::FromStr;

use nogo::io;
use nogo::game_logic::{self, validation};

/// Game entry point

fn main() {
    let args = io::get_game_arguments();

    match args.len() {
        3 => continue_saved_game(args),
        4 => start_new_game(args),
        _ => io::display_usage(),
    }
}

/// continue with the game specified in
/// the save file
fn continue_saved_game(args: Vec<String>) {}

/// start a fresh game with the
/// given specifications
fn start_new_game(args: Vec<String>) {
    game_logic::start_new_game(&args[0], &args[1], &args[2], &args[3]);
}
