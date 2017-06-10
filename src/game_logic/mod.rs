/// This module contains all the game-related logic

use std::collections::HashSet;
use error_handling as eh;
use io;

/// Some game constants

pub const MIN_BOARD_DIMENSION: i32 = 4;
pub const MAX_BOARD_DIMENSION: i32 = 1000;

const IR0: i32 = 1;
const IRX: i32 = 2;

const IC0: i32 = 4;
const ICX: i32 = 10;

const F0: i32 = 29;
const FX: i32 = 17;

const MOD_FACTOR: i32 = 10000003;

/// Game related data structures
#[derive(Debug, Copy, Clone)]
pub enum PlayerType {
    HUMAN,
    COMPUTER,
}

pub enum PlayerInput {
    Point(i32, i32),
    Quit(String),
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
    human: bool,
}

impl NogoPlayer {
    fn new(id: char, typ: PlayerType) -> Self {
        NogoPlayer {
            id: id,
            strings: Vec::new(),
            captured: false,
            human: match typ {
                PlayerType::COMPUTER => false,
                _ => true,
            },
        }
    }

    fn id(&self) -> &char {
        &self.id
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
    t: char,
}

impl Point {
    fn new(x: i32, y: i32, t: char) -> Self {
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

            loop {
                display_board(&board);

                {
                    update_board('0', &p1, &mut board);
                }

                display_board(&board);

                if let Some(player) = check_winner(&board) {
                    println!("Player {} wins!", player);
                    break;
                }

                {
                    update_board('X', &p2, &mut board);
                }

                if let Some(player) = check_winner(&board) {
                    println!("Player {} wins!", player);
                    break;
                }
            }

            println!("\nThank you for playing nogo!\n");
        }

        Err(e) => eh::exit_with_error(e),
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
    let coords = board.state.occupied();

    for i in 0..board.height {
        print!("|");

        for j in 0..board.width {
            let point = coords.iter().find(|&&t| (t.x, t.y) == (i, j));
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
    let (r, c) = match p_type {
        &PlayerType::COMPUTER => get_next_valid_move(&board, p_id),
        &PlayerType::HUMAN => io::get_player_move(&board, p_id),
    };

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


/// check if a winner can be established
/// to do this, the basic rules of the game
/// must be checked to see if any string
/// of either player has been captured
fn check_winner(board: &NogoBoard) -> Option<&char> {
    let free_points = board.state.liberties();
    let (p1, p2) = board.state.players();

    if p1.check_captured(&free_points) {
        display_board(&board);
        return Some(p2.id());
    } else if p2.check_captured(&free_points) {
        display_board(&board);
        return Some(p1.id());
    }

    None
}
