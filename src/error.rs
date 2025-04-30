use std::fmt;
use sophia::inmem::index::TermIndexFullError;

#[derive(Debug)]
pub enum ParseError {
    UnableInsertTriple(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
             ParseError::UnableInsertTriple(ref cause) => write!(f, "Unable to insert triple: {}", cause)
        }
    }
}

impl From<TermIndexFullError> for ParseError {
    fn from(err: TermIndexFullError) -> ParseError {
        ParseError::UnableInsertTriple(err.to_string())
    }
}