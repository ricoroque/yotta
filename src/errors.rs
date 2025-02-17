use core::fmt;
use crate::colors::*;

#[derive(Debug)]
pub enum YottaError {
    DivisionByZero,
    InverseDoesNotExist,
}

impl fmt::Display for YottaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YottaError::DivisionByZero => {
                write!(f, "{}{}[ERROR]{} Division by zero is not allowed.", BOLD, RED, RESET)
            }
            YottaError::InverseDoesNotExist => {
                write!(f, "{}{}[ERROR]{} Inverse does not exist.", BOLD, RED, RESET)
            }
        }
    }
}