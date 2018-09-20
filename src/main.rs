#![feature(rust_2018_preview)]

extern crate nogo;

use nogo::io;
use nogo::game_logic;


/// Game entry point
fn main() {
    let args = io::get_game_arguments();

    match args.len() {
        1 => continue_saved_game(&args[0]),
        4 => start_new_game(args),
        _ => io::display_usage(),
    }
}

/// start a fresh game with the
/// given specifications with relevant
/// validation done
fn start_new_game(args: Vec<String>) {
    println!("Welcome to nogo!\n");
    game_logic::start_new_game(&args[0], &args[1], &args[2], &args[3]);
}


/// continue the saved game as given
/// in the save file
fn continue_saved_game(file: &str) {
    println!("Welcome to nogo again!\n");
    game_logic::continue_saved_game(file);
}
