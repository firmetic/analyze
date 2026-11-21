use std::fmt;
use std::path::Path;

use crate::document::render;
use crate::evidence::Evidence;

#[derive(Debug)]
pub enum Error {
    Unwritable(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unwritable(error) => write!(formatter, "cannot write evidence.json: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Unwritable(error) => Some(error),
        }
    }
}

pub fn write(out: &Path, evidence: &Evidence) -> Result<(), Error> {
    std::fs::write(out.join("evidence.json"), render(evidence)).map_err(Error::Unwritable)
}
