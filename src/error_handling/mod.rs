/// This module is responsible for all error-handling related
/// tasks

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
    ErrorReadingGameFile,
    EOFWaitingForUserInput,
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

            NogoErrorKind::CantOpenFileForSaving => {
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


/// define an alias for NogoError for easier handling
pub type Result<'a, T> = ::std::result::Result<T, NogoError<'a>>;


/// The API

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
