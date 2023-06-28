use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatError {
    #[error("Io Error")]
    IoError(#[from] io::Error),
    #[error("Serenity Error")]
    SerenityError(#[from] serenity::Error),
    #[error("Sqlx")]
    SqlxError(#[from] sqlx::Error),
}

pub type CatResult<T> = Result<T, CatError>;
