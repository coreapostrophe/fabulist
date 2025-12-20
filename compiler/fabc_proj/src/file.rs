use std::fs::{File as StdFile, OpenOptions};
use std::path::Path;

use crate::error::Error;

pub struct File;

impl File {
    pub fn read(path: &Path) -> Result<StdFile, Error> {
        Ok(OpenOptions::new().read(true).open(path)?)
    }
}
