pub mod document;
pub mod evidence;
pub mod input;
pub mod output;
pub mod section;

use std::path::Path;

pub fn run(elf: &Path, out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read(elf)?;
    let evidence = input::load(&bytes)?;

    output::write(out, &evidence)?;

    Ok(())
}
