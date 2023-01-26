use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatError {
    #[error("Token could not be read, please add [export catbot-token = <token>] in your .zshrc/.bashrc")]
    CannotReadToken,
    #[error("Bot could not be built")]
    CannotBuildBot,
    #[error("Bot could not start")]
    CannotStartBot,
}

pub type CatResult<T> = Result<T, CatError>;
