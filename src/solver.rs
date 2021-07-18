use crate::year2020;
use std::{
    collections::HashMap,
    error,
    fmt::{self, Display, Formatter},
    result,
};

/// An error that occurs while solving a puzzle.
#[derive(Debug)]
pub struct Error {
    source: Option<Box<dyn error::Error + Send + Sync>>,
    desc: String,
}

impl Error {
    /// Constructs a new `Error` that is the result of another error.
    pub fn new(source: impl error::Error + Send + Sync + 'static, desc: impl Into<String>) -> Self {
        Self {
            source: Some(Box::new(source)),
            desc: desc.into(),
        }
    }

    /// Constructs a new `Error` from a description.
    pub fn from_desc(desc: impl Into<String>) -> Self {
        Error {
            source: None,
            desc: desc.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            Some(e) => Some(e.as_ref()),
            None => None,
        }
    }
}

/// The result of solving a puzzle.
pub type Result = result::Result<Vec<String>, Error>;

/// A function that solves a puzzle.
pub type Solver = fn(&str) -> Result;

/// Returns a map that maps a day and year to a solver that solves the puzzle for that day and year.
pub fn all_puzzles() -> HashMap<(i32, u32), Solver> {
    let mut puzzles: HashMap<(i32, u32), Solver> = HashMap::new();
    puzzles.insert((2020, 1), year2020::day01::solve);
    puzzles.insert((2020, 2), year2020::day02::solve);
    puzzles.insert((2020, 3), year2020::day03::solve);
    puzzles
}
