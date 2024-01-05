use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ImagenGeneration {
    Color(#[from] ParseColor),
    Background(#[from] Background),
}

#[derive(Debug, Error)]
pub enum Background {
    #[error("Cannot Parse Background from String")]
    CannotParse,
    #[error("Invalid format of String")]
    InvalidFormat,
    #[error("Invalid path of image")]
    InvalidPath,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseColor {
    #[error("Invalid length of String")]
    InvalidLength,
    #[error("Invalid digit")]
    InvalidDigit,
    #[error("Error parsing number")]
    Parse(#[from] ParseIntError),
}
