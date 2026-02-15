use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("db error: {0}")]
  Db(String),

  #[error("invalid input: {0}")]
  Invalid(String),

  #[error("internal error: {0}")]
  Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;
