/// This module contains all the game-related logic

pub mod validation;

use error_handling;

pub enum PlayerType {
    HUMAN,
    COMPUTER,
}


/// Start a fresh game
pub fn start_new_game<'a>(p1: &'a str,
                          p2: &'a str,
                          height: &'a str,
                          width: &'a str)
                          -> error_handling::Result<'a, ()> {

    Ok(())
}

/// Load a saved game
pub fn continue_saved_game<'a>(save_file: &'a str) -> error_handling::Result<'a, ()> {
    Ok(())
}
