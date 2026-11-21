use std::fmt;

use object::elf::FileHeader32;
use object::read::elf::FileHeader;

use crate::evidence::Evidence;
use crate::section::Section;

#[derive(Debug)]
pub struct Error(object::read::Error);

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid ELF: {}", self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

pub fn load(bytes: &[u8]) -> Result<Evidence, Error> {
    let header = FileHeader32::<object::Endianness>::parse(bytes).map_err(Error)?;
    let endian = header
        .endian()
        .expect("a parsed header has a valid byte order");
    let headers = header.section_headers(endian, bytes).map_err(Error)?;

    Ok(Evidence::new(
        headers
            .iter()
            .enumerate()
            .map(|(index, _)| Section::new(u32::try_from(index).expect("an index fits u32")))
            .collect(),
    ))
}
