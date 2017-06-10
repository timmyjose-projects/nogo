/// All the I/O and visual rendering of the game
/// is handled by this module

use std::io::{self, Write, BufWriter};
use std::fs::File;
use std::str::FromStr;

use game_logic as gl;
use error_handling as eh;


/// Get the command line arguments for the
/// game
pub fn get_game_arguments() -> Vec<String> {
    ::std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
}

/// display the correct usage of
/// the game
pub fn display_usage() {
    writeln!(io::stderr(),
             "Usage: nogo p1type p2type [height width | filename]")
        .unwrap();
    eh::exit_with_error(eh::construct_error("insufficient number of arguments",
                                            eh::NogoErrorKind::IncorrectNumberOfArgs));
}

/// ensure that only a proper move is allowed
/// in case the player has entered 'w' followed by a path
/// then save the game and quit.
pub fn get_player_move(board: &gl::NogoBoard, player_name: char) -> gl::PlayerInput {
    loop {
        print!("Player {}> ", player_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let entries = input.trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if entries.len() < 1 || entries.len() > 2 {
            continue;
        }

        // check if the player wants to save the game
        if entries.len() == 1 {
            let c = entries[0].chars().nth(0).unwrap();

            match c {
                'w' | 'W' => {
                    let path = String::from(&entries[0][1..]);

                    if path.len() == 0 {
                        continue;
                    } else {
                        return gl::PlayerInput::Quit(path);
                    }
                }
                _ => continue,
            }
        }

        // check for (row, column) input
        let r = match i32::from_str(entries[0].trim()) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let c = match i32::from_str(entries[1].trim()) {
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

        return gl::PlayerInput::Point(r, c);
    } // loop
}


/// save the game to the given save file
pub fn save_game_state<'a>(path: String, data: Vec<String>) -> eh::Result<'a, ()> {
    let mut writer = BufWriter::new(File::create(path)?);

    for line in data.iter() {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
