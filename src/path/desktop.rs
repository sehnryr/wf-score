use std::path::PathBuf;

use super::error::Error;
use crate::APP_IDENTIFIER;

pub fn app_data_local_dir() -> Result<PathBuf, Error> {
    dirs::data_local_dir()
        .ok_or(Error::UnknownPath)
        .map(|dir| dir.join(APP_IDENTIFIER))
}
