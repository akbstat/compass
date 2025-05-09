use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Errors>;
