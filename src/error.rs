use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Db(#[from] sled::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error("api with name {0} already exists")]
    ApiAlreadyExists(String),
    #[error("cell already set")]
    CellSetError,
}

pub type Result<T> = std::result::Result<T, Error>;
