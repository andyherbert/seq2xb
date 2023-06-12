use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Seq2XBinError {
    InvalidColumnValue,
}

impl Display for Seq2XBinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seq2XBinError::InvalidColumnValue => write!(f, "Invalid Column Value"),
        }
    }
}

impl Error for Seq2XBinError {}
