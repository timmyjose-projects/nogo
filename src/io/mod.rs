/// All the I/O and visual rendering of the game
/// is handled by this module

use std::io::{self, Write};

use ::error_handling;

pub fn get_game_arguments() -> Vec<String> {
    ::std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
}

pub fn display_usage() {
    writeln!(io::stderr(), "Usage: nogo p1type p2type [height width | filename]").unwrap();
    error_handling::exit_with_status(1);
}
