use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatError {
    #[error("Io Error")]
    Io(#[from] io::Error),
    #[error("Serenity Error")]
    Serenity(#[from] serenity::Error),
    #[error("Sqlx")]
    Sqlx(#[from] sqlx::Error),
}

pub type CatResult<T> = Result<T, CatError>;
