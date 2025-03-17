use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("serde_yml error: {}", _0.to_string())]
    SerdeYmlError(#[from] serde_yml::Error),
    #[error("io error: {}", _0.to_string())]
    IoError(#[from] io::Error),
}
