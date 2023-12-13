use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("unknown error")]
    Unknown,
}
