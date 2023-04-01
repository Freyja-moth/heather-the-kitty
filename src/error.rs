use std::io;

use thiserror::Error;
use toml::{de, ser};

#[derive(Error, Debug)]
pub enum CatError {
    #[error("Io Error")]
    IoError(#[from] io::Error),
    #[error("Serialize Error")]
    SerError(#[from] ser::Error),
    #[error("Deserialize Error")]
    DeError(#[from] de::Error),
    #[error("Serenity Error")]
    SerenityError(#[from] serenity::Error),
}

pub type CatResult<T> = Result<T, CatError>;
