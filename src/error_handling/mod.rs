/// This module is responsible for all error-handling related
/// tasks

pub mod validation;

use std::io::{self, Write};
use std::error::Error;
use std::fmt;


/// Define a new type for nogo errors
#[derive(Debug)]
pub enum NogoErrorKind {
    IncorrectNumberOfArgs,
    IncorrectTypes,
    InvalidBoardDimensions,
    CantOpenFileForSaving,
    CantOpenFileForReading,
    ErrorReadingGameFile,
    EOFWaitingForUserInput,
    SystemIOError,
    ParsingError,
}

#[derive(Debug)]
pub struct NogoError<'a> {
    kind: NogoErrorKind,
    status: i32,
    general: &'a str,
    specific: &'a str,
}

impl<'a> fmt::Display for NogoError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.specific)
    }
}

impl<'a> Error for NogoError<'a> {
    fn description(&self) -> &str {
        self.specific
    }
}

impl<'a> NogoError<'a> {
    pub fn new(kind: NogoErrorKind) -> Self {
        let mut error = NogoError {
            kind: kind,
            status: 0,
            general: "",
            specific: "",
        };

        match error.kind {
            NogoErrorKind::IncorrectNumberOfArgs => {
                error.status = 1;
                error.general = "Program started with incorrect number of arguments";
            }

            NogoErrorKind::IncorrectTypes => {
                error.status = 2;
                error.general = "Invalid type(s)";
            }

            NogoErrorKind::InvalidBoardDimensions => {
                error.status = 3;
                error.general = "Invalid board dimension(s)";
            }

            NogoErrorKind::CantOpenFileForReading => {
                error.status = 4;
                error.general = "Unable to open save file";
            }

            NogoErrorKind::ErrorReadingGameFile => {
                error.status = 5;
                error.general = "Incorrect save file contents";
            }

            NogoErrorKind::EOFWaitingForUserInput => {
                error.status = 6;
                error.general = "End of input from user";
            }

            NogoErrorKind::CantOpenFileForSaving => {
                error.status = 7;
                error.general = "Unable to open new save file";
            }

            NogoErrorKind::SystemIOError => {
                error.status = 8;
                error.general = "System IO error";
            }

            NogoErrorKind::ParsingError => {
                error.status = 9;
                error.general = "Error while parsing value";
            }
        }

        error
    }

    pub fn kind(&self) -> &NogoErrorKind {
        &self.kind
    }

    pub fn status(&self) -> i32 {
        self.status
    }

    pub fn general(&self) -> &'a str {
        &self.general
    }

    pub fn specific(&self) -> &'a str {
        &self.specific
    }

    pub fn set_specific(&mut self, msg: &'a str) {
        self.specific = msg;
    }
}

/// to allow conversion from io::Error into NogoError
impl<'a> ::std::convert::From<io::Error> for NogoError<'a> {
    fn from(_: io::Error) -> Self {
        NogoError::new(NogoErrorKind::SystemIOError)
    }
}

/// to allow conversion from std::num::ParseIntError into NogoError
impl<'a> ::std::convert::From<::std::num::ParseIntError> for NogoError<'a> {
    fn from(_: ::std::num::ParseIntError) -> Self {
        NogoError::new(NogoErrorKind::ParsingError)
    }
}


/// define an alias for NogoError for easier handling
pub type Result<'a, T> = ::std::result::Result<T, NogoError<'a>>;


/// The API

/// normal exit
pub fn clean_exit() {
    println!("\nThank you for playing nogo!\n");
    ::std::process::exit(0);
}

/// exit with error code
pub fn exit_with_error(error: NogoError) {
    writeln!(io::stderr(), "Error: {}", error.description()).unwrap();
    ::std::process::exit(error.status());
}


/// helper method to throw a properly constructed error object
pub fn construct_error(specific: &str, kind: NogoErrorKind) -> NogoError {
    let mut err = NogoError::new(kind);
    err.set_specific(specific);
    err
}
