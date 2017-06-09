/// All the I/O and visual rendering of the game
/// is handled by this module

use std::io::{self, Write};
use std::str::FromStr;

use game_logic::NogoBoard;
use error_handling as eh;

/// Get the command line arguments for the
/// game
pub fn get_game_arguments() -> Vec<String> {
    ::std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
}


pub fn display_usage() {
    writeln!(io::stderr(),
             "Usage: nogo p1type p2type [height width | filename]")
        .unwrap();
    eh::exit_with_error(eh::NogoError::new(eh::NogoErrorKind::IncorrectNumberOfArgs));
}

/// ensure that only a proper move is allowed
pub fn get_player_move(board: &NogoBoard, player_name: char) -> (i32, i32) {
    let mut r;
    let mut c;

    loop {
        print!("Player {}> ", player_name);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let entries = input.trim().split_whitespace().collect::<Vec<_>>();

        if entries.len() != 2 {
            continue;
        }

        r = match i32::from_str(entries[0].trim()) {
            Ok(val) => val,
            Err(_) => continue,
        };

        c = match i32::from_str(entries[1].trim()) {
            Ok(val) => val,
            Err(_) => continue,
        };

        if r >= board.height() || r < 0 {
            continue;
        }

        if c >= board.width() || c < 0 {
            continue;
        }
        
        // game-logic related validation here
        if !eh::validation::validate_user_move(board, (r, c)) {
            continue;
        }

        return (r, c);
    }
}
