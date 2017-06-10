/// All the I/O and visual rendering of the game
/// is handled by this module

use std::io::{self, Write, BufWriter, BufRead, BufReader};
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


/// The player can enter either a (row, column) pair, or
/// w[full-path-to-save-file]. Handle either situation
/// with proper validation
pub fn get_player_move(board: &gl::NogoBoard, player_name: char) -> gl::PlayerInput {
    let mut r;
    let mut c;

    loop {
        print!("Player {}> ", player_name);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let entries = input.trim()
            .split_whitespace()
            .collect::<Vec<_>>();

        // check if the user wants to save the game
        if entries.len() == 1 {
            match entries[0].trim().chars().nth(0) {
                Some('w') | Some('W') => {
                    let path =
                        String::from_utf8(entries[0].trim().bytes().skip(1).collect::<Vec<_>>())
                            .unwrap();
                    if path.len() != 0 {
                        return gl::PlayerInput::Save(path);
                    }
                }

                _ => continue,
            }
        }

        if entries.len() > 2 {
            continue;
        }

        // check for (row, column) input
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

        return gl::PlayerInput::Point(r, c);
    } // loop
}


/// save the game to the given save file
pub fn save_game_state<'a>(path: &str, data: Vec<String>) -> eh::Result<'a, ()> {
    let mut writer = BufWriter::new(File::create(path)?);

    for line in data.iter() {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}


/// load the game state to a vector of lines
pub fn load_game_state<'a>(path: &str) -> eh::Result<'a, Vec<String>> {
    let mut data = Vec::new();
    let reader = BufReader::new(File::open(path)?);

    for line in reader.lines() {
        let line = line?;

        data.push(line);
    }

    Ok(data)
}

/// parse the saved file metadata to reconstruct the game
/// state
pub fn parse_save_file_metadata<'a>
    (metadata: &Vec<&str>)
     -> eh::Result<'a, (i32, i32, gl::PlayerType, gl::PlayerType, char)> {
    Ok((i32::from_str(&metadata[0])?,
        i32::from_str(&metadata[1])?,
        get_player_type(&metadata[2])?,
        get_player_type(&metadata[3])?,
        metadata[4].chars().nth(0).unwrap()))
}

fn get_player_type<'a>(p: &str) -> eh::Result<'a, gl::PlayerType> {
    match p {
        "c" | "C" => Ok(gl::PlayerType::COMPUTER),
        "h" | "H" => Ok(gl::PlayerType::HUMAN),
        _ => {
            return Err(eh::construct_error("incorrect type for player 0",
                                           eh::NogoErrorKind::IncorrectTypes));
        }
    }
}

fn get_current_player_id<'a>(p_id: &str) -> eh::Result<'a, char> {
    match p_id {
        "0" => Ok(gl::PLAYER_ZERO),
        "x" | "X" => Ok(gl::PLAYER_ONE),
        _ => {
            return Err(eh::construct_error("incorrect type specified for current player",
                                           eh::NogoErrorKind::IncorrectTypes));
        }
    }
}


/// parse the rest of the save file to generate
/// a pair of points for both players
pub fn parse_player_strings_from_saved_file<'a>
    (data: &Vec<&String>)
     -> eh::Result<'a, (Vec<gl::Point>, Vec<gl::Point>)> {

    let (mut zero_points, mut x_points) = (Vec::new(), Vec::new());
    let (mut i, mut j) = (0, 0);

    for line in data.iter() {
        for c in line.chars() {
            match c {
                '0' => zero_points.push(gl::Point::new(i, j, gl::PLAYER_ZERO)),

                'x' | 'X' => x_points.push(gl::Point::new(i, j, gl::PLAYER_ONE)),

                '.' => {}

                _ => {
                    return Err(eh::construct_error("invalid character found in board data",
                                                   eh::NogoErrorKind::IncorrectTypes));
                }
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }

    Ok((zero_points, x_points))
}
