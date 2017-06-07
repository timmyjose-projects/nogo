/// This module is responsible for all error-handling related
/// tasks

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
    val: i32,
    message: &'a str,
}

impl<'a> fmt::Display for NogoError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> Error for NogoError<'a> {
    fn description(&self) -> &str {
        self.message
    }
}

impl<'a> NogoError<'a> {
    fn new(kind: NogoErrorKind) -> Self {
        let mut error = NogoError {
            kind: kind,
            val: 0,
            message: "",
        };

        match error.kind {
            NogoErrorKind::IncorrectNumberOfArgs => {
                error.val = 1;
                error.message = "Program started with incorrect number of arguments";
            }

            NogoErrorKind::IncorrectTypes => {
                error.val = 2;
                error.message = "Invalid type(s)";
            }

            NogoErrorKind::InvalidBoardDimensions => {
                error.val = 3;
                error.message = "Invalid board dimension(s)";
            }

            NogoErrorKind::CantOpenFileForSaving => {
                error.val = 4;
                error.message = "Unable to open save file";
            }

            NogoErrorKind::ErrorReadingGameFile => {
                error.val = 5;
                error.message = "Incorrect save file contents";
            }

            NogoErrorKind::EOFWaitingForUserInput => {
                error.val = 6;
                error.message = "End of input from user";
            }
        }

        error
    }
}


/// define an alias for NogoError for easier handling
pub type Result<'a, T> = ::std::result::Result<T, NogoError<'a>>;


/// The API

pub fn exit_with_status(status: i32) {
    ::std::process::exit(status);
}
