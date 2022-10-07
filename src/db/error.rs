use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DbError {
    pub details: String
}

impl DbError {
    pub fn new(msg: String) -> DbError {
        DbError{details: msg.to_string()}
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        &self.details
    }
}