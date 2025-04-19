use crate::path::error::Error as PathError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Path error: {0}")]
    Path(#[from] PathError),
}
