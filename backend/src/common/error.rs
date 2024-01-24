#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DBError(#[from] sqlx::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
