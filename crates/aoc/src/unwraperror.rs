use std::error::Error;

#[derive(Debug)]
pub struct UnwrapError {}

impl std::fmt::Display for UnwrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unwrap error")
    }
}

impl Error for UnwrapError {}

pub const DAMN: UnwrapError = UnwrapError {};
