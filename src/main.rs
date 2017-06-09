extern crate nogo;

use nogo::io;
use nogo::game_logic;


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
/// the save file, with proper validation
/// of the file contents
fn continue_saved_game(args: Vec<String>) {
    
}

/// start a fresh game with the
/// given specifications with relevant
/// validation done
fn start_new_game(args: Vec<String>) {
   game_logic::start_new_game(&args[0], &args[1], &args[2], &args[3]);
}
