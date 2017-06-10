/// This module contains all the game-related logic

use std::collections::HashSet;
use error_handling as eh;
use io;


/// Some game constants

pub const MIN_BOARD_DIMENSION: i32 = 4;
pub const MAX_BOARD_DIMENSION: i32 = 1000;

pub const PLAYER_ZERO: char = '0';
pub const PLAYER_ONE: char = 'X';


/// Constants for computer-generated
/// moves

const IR0: i32 = 1;
const IRX: i32 = 2;

const IC0: i32 = 4;
const ICX: i32 = 10;

const F0: i32 = 29;
const FX: i32 = 17;

const MOD_FACTOR: i32 = 10000003;

/// Game related data structures
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerType {
    HUMAN,
    COMPUTER,
    NONE, // only for validation
}

pub enum PlayerInput {
    Point(i32, i32),
    Save(String),
}

/// the overall board -it holds state, but does
/// not really do any processng on its own
#[derive(Debug)]
pub struct NogoBoard {
    height: i32,
    width: i32,
    state: NogoBoardState,
}

impl NogoBoard {
    fn new(p1: PlayerType, p2: PlayerType, h: i32, w: i32) -> Self {
        NogoBoard {
            height: h,
            width: w,
            state: NogoBoardState::new(p1, p2, (h, w)),
        }
    }

    fn player(&mut self, id: char) -> Option<&mut NogoPlayer> {
        match self.state.players {
            (ref mut p1, ref mut p2) => {
                if p1.id == id {
                    Some(p1)
                } else if p2.id == id {
                    Some(p2)
                } else {
                    None
                }
            }
        }
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn liberties(&self) -> HashSet<Point> {
        self.state.liberties()
    }

    pub fn update_occupied(&mut self, p: Point) {
        self.state.update_occupied(p);
    }
}

/// this holds the game state by holding
/// references to the current players of
/// the game. `all_points` is merely
/// used for performance reasons
#[derive(Debug)]
struct NogoBoardState {
    players: (NogoPlayer, NogoPlayer),
    all_points: HashSet<Point>,
    occupied_points: HashSet<Point>,
}

impl NogoBoardState {
    fn new(p1: PlayerType, p2: PlayerType, limits: (i32, i32)) -> Self {
        let mut state = NogoBoardState {
            players: (NogoPlayer::new('0', p1), NogoPlayer::new('X', p2)),
            all_points: HashSet::new(),
            occupied_points: HashSet::new(),
        };

        for i in 0..limits.0 {
            // height
            for j in 0..limits.1 {
                // width
                let point = Point::new(i, j, '.');
                state.all_points.insert(point.clone());
            }
        }

        state
    }

    fn players(&self) -> (&NogoPlayer, &NogoPlayer) {
        (&self.players.0, &self.players.1)
    }

    /// retrieve all the current liberties
    /// custom retrieval since the default
    /// HashSet implementation does not allow
    /// us to check against a custom comparator
    pub fn liberties(&self) -> HashSet<Point> {
        let mut free = HashSet::new();

        for point in self.all_points.iter() {
            if self.occupied().iter().find(|&&p| (p.x, p.y) == (point.x, point.y)).is_none() {
                free.insert(*point);
            }
        }

        free
    }

    /// retrieve the current occupied points
    /// of the board
    fn occupied(&self) -> &HashSet<Point> {
        &self.occupied_points
    }

    /// update the occupied points of the board
    /// with the new point
    fn update_occupied(&mut self, p: Point) {
        self.occupied_points.insert(p);
    }
}

/// this represents a player in the game.
/// each player holds the vector of "strings"
/// that he/she owns
#[derive(Debug)]
struct NogoPlayer {
    id: char,
    strings: Vec<NogoString>,
    captured: bool,
    kind: PlayerType,
}

impl NogoPlayer {
    fn new(id: char, typ: PlayerType) -> Self {
        NogoPlayer {
            id: id,
            strings: Vec::new(),
            captured: false,
            kind: typ,
        }
    }

    fn id(&self) -> char {
        self.id
    }

    fn kind(&self) -> &PlayerType {
        &self.kind
    }

    // for each string of the current player, check
    // if the new coordinates form part of an existing
    // string. If so, update the string data, otherwise
    // add a new string
    fn update_strings(&mut self, pt: Point) {
        let (x, y) = (pt.x, pt.y);

        for string in self.strings.iter_mut() {
            let (l, r, u, d) = ((x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y));

            if string.find(l) || string.find(u) || string.find(r) || string.find(d) {
                string.add(pt);
                return;
            }
        }

        // no match, so add a new string
        self.strings.push(NogoString::new(pt));
    }

    // check if, after the last move, this
    // player has any of its strings captured -
    // if none of the components of a string
    // has any liberties, then the string, and
    // therefore the player is captured
    fn check_captured(&self, free: &HashSet<Point>) -> bool {
        for string in self.strings.iter() {
            let mut count = 0;
            for component in string.components.iter() {
                let (r, c) = (component.x, component.y);

                let (l, r, u, d) = ((r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c));

                if self.find_point(&l, free) || self.find_point(&r, free) ||
                   self.find_point(&u, free) || self.find_point(&d, free) {
                    count += 1;
                }
            }

            if count == 0 {
                return true;
            }
        }

        false
    }

    fn find_point(&self, needle: &(i32, i32), haystack: &HashSet<Point>) -> bool {
        match haystack.iter().find(|p| (p.x, p.y) == *needle) {
            Some(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NogoString {
    components: Vec<Point>,
}

impl NogoString {
    fn new(p: Point) -> Self {
        let mut string = NogoString { components: Vec::new() };
        string.components.push(p);
        string
    }

    fn find(&self, t: (i32, i32)) -> bool {
        self.components.iter().find(|&&p| (p.x, p.y) == (t.0, t.1)).is_some()
    }

    fn add(&mut self, p: Point) {
        self.components.push(p);
    }
}

/// represents a point in the board
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
    t: char,
}

impl Point {
    pub fn new(x: i32, y: i32, t: char) -> Self {
        Point { x: x, y: y, t: t }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn t(&self) -> &char {
        &self.t
    }
}


///
/// Start a fresh game
///
pub fn start_new_game<'a>(p1: &'a str, p2: &'a str, height: &'a str, width: &'a str) {
    /// check if the arguments are correct
    match eh::validation::validate_new_game_parameters(p1, p2, height, width) {
        Ok((p1, p2, h, w)) => {
            let mut board = create_board(&p1, &p2, h, w);
            game_loop(&p1, &p2, PLAYER_ZERO, &mut board);
        }

        Err(e) => eh::exit_with_error(e),
    }
}

/// factoring out the game loop so that it can be used with
/// both a new game as well as continuing from a saved game
fn game_loop(p1: &PlayerType, p2: &PlayerType, start_player: char, board: &mut NogoBoard) {
    let first_player_type = if start_player == PLAYER_ZERO { p1 } else { p2 };
    let second_player_type = if first_player_type == p1 { p2 } else { p1 };

    let other_player = if start_player == PLAYER_ZERO { PLAYER_ONE } else { PLAYER_ZERO };
    
    loop {
        display_board(&board);

        {
            update_board(start_player, &first_player_type, board);
        }

        display_board(&board);

        check_winner(&board);

        {
            update_board(other_player, &second_player_type, board);
        }

        check_winner(&board);
    }
} // game loop


/// Continue the game as saved in the save file.
/// The way this will work is as follows -
/// load the game metadata from the saved file,
/// and recreate the state of the game from the
/// board positions. Then continue the game
/// from that juncture, with the player who had
/// saved the file getting the first move in the
/// new game
pub fn continue_saved_game(save_file: &str) {
    let mut game_data = Vec::new(); // just to satisfy the damned checker

    match io::load_game_state(save_file) {
        Ok(data) => game_data = data,
        Err(e) => eh::exit_with_error(e),
    }

    // load the metadata
    let metadata = &game_data[0]
        .split_whitespace()
        .collect::<Vec<_>>();

    let (mut height, mut width, mut p1type, mut p2type, mut curr_player) =
        (0, 0, PlayerType::NONE, PlayerType::NONE, ' ');

    match io::parse_save_file_metadata(metadata) {
        Ok((h, w, p1, p2, c)) => {
            height = h;
            width = w;
            p1type = p1;
            p2type = p2;
            curr_player = c;
        }

        Err(e) => eh::exit_with_error(e),
    }

    // recreate the game state
    let mut board = create_board(&p1type, &p2type, height, width);
    let game_data = game_data.iter().skip(1).collect::<Vec<_>>();
    let (mut player_0_strings, mut player_1_strings) = (Vec::new(), Vec::new());

    match io::parse_player_strings_from_saved_file(&game_data) {
        Ok((p1_strs, p2_strs)) => {
            player_0_strings = p1_strs;
            player_1_strings = p2_strs;
        }
        Err(e) => eh::exit_with_error(e),
    }

    // replay the game moves till the current point
    replay_moves(PLAYER_ZERO, player_0_strings, &mut board);
    replay_moves(PLAYER_ONE, player_1_strings, &mut board);

    // continue the game
    game_loop(&p1type, &p2type, curr_player, &mut board);
}


fn replay_moves(player: char, moves: Vec<Point>, board: &mut NogoBoard) {
    for mov in moves.iter() {
        board.update_occupied(*mov);
        // inefficiency here due to Rust's demented checker
        board.player(player).unwrap().update_strings(*mov);
    }
}


///
/// Game logic related functions
///

/// create a fresh board with the given dimensions
fn create_board(p1: &PlayerType, p2: &PlayerType, h: i32, w: i32) -> NogoBoard {
    NogoBoard::new(*p1, *p2, h, w)
}

/// display the current state of the board
fn display_board(board: &NogoBoard) {
    print_head(board.width);
    print_rows(board);
    print_tail(board.width);
}

fn print_head(n: i32) {
    print!("/");

    for _ in 0..n {
        print!("-");
    }
    println!("\\");
}

/// collect the string coordinates for both players
/// in one go so that a single pass will be sufficient
/// to display the board
fn print_rows(board: &NogoBoard) {
    let points = board.state.occupied();

    for i in 0..board.height {
        print!("|");

        for j in 0..board.width {
            let point = points.iter().find(|&&t| (t.x, t.y) == (i, j));
            if let Some(val) = point {
                print!("{}", val.t);
            } else {
                print!(".");
            }
        }
        println!("|");
    }

}

fn print_tail(n: i32) {
    print!("\\");

    for _ in 0..n {
        print!("-");
    }
    println!("/\n");
}

///
/// update the board state with a player move.
/// the player can be a computer or a human -
/// accept input or generate moves accordingly
fn update_board(p_id: char, p_type: &PlayerType, board: &mut NogoBoard) {
    if p_type == &PlayerType::HUMAN {
        let player_input = io::get_player_move(&board, p_id);

        match player_input {
            PlayerInput::Save(path) => save_game_and_exit(&path, &board, p_id),
            PlayerInput::Point(x, y) => update_board_with_move(p_id, x, y, board),
        }
    } else {
        let (x, y) = get_next_valid_move(&board, p_id);
        update_board_with_move(p_id, x, y, board);
    }
}


fn update_board_with_move(p_id: char, r: i32, c: i32, board: &mut NogoBoard) {
    let point = Point::new(r, c, p_id);

    board.update_occupied(point.clone());

    {
        let player = board.player(p_id).unwrap();

        // update the strings of the player
        player.update_strings(point);
    }
}


/// generate the moves for the computer as per
/// the given algorithm. this will loop until
/// a valid move is found
fn get_next_valid_move(board: &NogoBoard, p: char) -> (i32, i32) {
    let ir = if p == '0' { IR0 } else { IRX };
    let ic = if p == '0' { IC0 } else { ICX };
    let f = if p == '0' { F0 } else { FX };

    let gw = board.width();
    let gh = board.height();

    let mut r = ir;
    let mut c = ic;
    let b = ir * gw + ic;

    let mut m = 0;
    let mut n;

    loop {
        m += 1;

        let (mut x, mut y) = match m % 5 {
            0 => {
                n = (b + m / 5 * f) % MOD_FACTOR;
                r = n / gw;
                c = n % gw;
                (r, c)
            }

            1 => {
                r += 1;
                c += 1;
                (r, c)
            }

            2 => {
                r += 2;
                c += 1;
                (r, c)
            }

            3 => {
                r += 1;
                (r, c)
            }

            4 => {
                c += 1;
                (r, c)
            }

            _ => (r, c),
        };

        x %= gh;
        y %= gw;

        if eh::validation::validate_user_move(board, (x, y)) {
            println!("Player {}: {} {}", p, x, y);
            return (x, y);
        }
    }
}


/// Save the game - first retrieve the current game state in proper form
/// and then save it to the save file. Then quite the game gracefully
fn save_game_and_exit(save_file: &str, board: &NogoBoard, curr_player: char) {
    let game_data = get_current_game_state(&board, curr_player);

    match io::save_game_state(save_file, game_data) {
        Ok(_) => {
            println!("\nFinished saving current game state to file {}", save_file);
            eh::clean_exit();
        }
        Err(e) => eh::exit_with_error(e),
    }
}

/// format of the save file -
/// metadata: h w p1type p2type pturn
/// newline
/// board state
/// newline
fn get_current_game_state(board: &NogoBoard, curr_player: char) -> Vec<String> {
    let mut data = Vec::new();

    // metadata
    data.push(format!("{} {} {} {} {}",
                      board.height(),
                      board.width(),
                      if board.state.players().0.kind() == &PlayerType::HUMAN {
                          'h'
                      } else {
                          'c'
                      },
                      if board.state.players().1.kind() == &PlayerType::HUMAN {
                          'h'
                      } else {
                          'c'
                      },
                      curr_player));

    // actual board (sans borders)
    let mut line;
    let points = board.state.occupied();

    for i in 0..board.height {
        line = String::new();
        for j in 0..board.width {
            let point = points.iter().find(|&&t| (t.x, t.y) == (i, j));
            if let Some(val) = point {
                line.push(val.t);
            } else {
                line.push('.');
            }
        }
        data.push(line);
    }
    data
}



/// check if a winner can be established
/// to do this, the basic rules of the game
/// must be checked to see if any string
/// of either player has been captured
fn check_winner(board: &NogoBoard) {
    let free_points = board.state.liberties();
    let (p1, p2) = board.state.players();

    if p1.check_captured(&free_points) {
        display_board(&board);
        println!("Player {} wins!", p2.id());
        eh::clean_exit();
    } else if p2.check_captured(&free_points) {
        display_board(&board);
        println!("Player {} wins!", p1.id());
        eh::clean_exit();
    }
}
