#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown path")]
    UnknownPath,
}
