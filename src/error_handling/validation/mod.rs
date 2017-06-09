/// This submodule will handle all he validation for game input
/// as well as the saved file format.

use std::str::FromStr;
use error_handling as eh;
use game_logic as gl;


///
/// validate the command line arguments for a new
/// game
///
pub fn validate_new_game_parameters<'a>
    (p1: &'a str,
     p2: &'a str,
     height: &'a str,
     width: &'a str)
     -> eh::Result<'a, (gl::PlayerType, gl::PlayerType, i32, i32)> {

    let p1type = match validate_player_type(p1) {
        Ok(typ) => typ,
        Err(e) => return Err(e),
    };

    let p2type = match validate_player_type(p2) {
        Ok(typ) => typ,
        Err(e) => return Err(e),
    };

    let (height, width) = match validate_board_dimensions(height, width) {
        Ok((h, w)) => (h, w),
        Err(e) => return Err(e),
    };

    Ok((p1type, p2type, height, width))
}

fn validate_player_type<'a>(pt: &'a str) -> eh::Result<'a, gl::PlayerType> {
    match pt {
        "c" | "C" => Ok(gl::PlayerType::COMPUTER),
        "h" | "H" => Ok(gl::PlayerType::HUMAN),
        _ => {
            Err(eh::construct_error("wrong input for player type - only 'c' or 'h' accepted",
                                    eh::NogoErrorKind::IncorrectTypes))
        }
    }
}


fn validate_board_dimensions<'a>(height: &'a str, width: &'a str) -> eh::Result<'a, (i32, i32)> {
    let (h, w): (i32, i32);

    if let Ok(val) = i32::from_str(height) {
        if val < gl::MIN_BOARD_DIMENSION || val > gl::MAX_BOARD_DIMENSION {
            return Err(eh::construct_error("height must be between 4 and 1000 (inclusive)",
                                           eh::NogoErrorKind::InvalidBoardDimensions));
        }

        h = val;
    } else {
        return Err(eh::construct_error("incorrect type of height - only a number is accepted",
                                       eh::NogoErrorKind::IncorrectTypes));
    }

    if let Ok(val) = i32::from_str(width) {
        if val < gl::MIN_BOARD_DIMENSION || val > gl::MAX_BOARD_DIMENSION {
            return Err(eh::construct_error("width must be between 4 and 1000 (inclusive)",
                                           eh::NogoErrorKind::InvalidBoardDimensions));
        }

        w = val;
    } else {
        return Err(eh::construct_error("incorrect type of width - only a number is accepted",
                                       eh::NogoErrorKind::IncorrectTypes));
    }

    Ok((h, w))
}


/// Validate the player move against the existing
/// state of the board
pub fn validate_user_move(board: &gl::NogoBoard, mv: (i32, i32)) -> bool {
    let points = board.liberties();

    for point in points.iter() {
        if (point.x(), point.y()) == mv {
            return true;
        }
    }
    false
}
