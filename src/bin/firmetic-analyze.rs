use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(elf) = std::env::args_os().nth(1) else {
        eprintln!("usage: firmetic-analyze <elf>");
        return ExitCode::FAILURE;
    };

    match firmetic_analyze::run(Path::new(&elf), Path::new(".")) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("firmetic-analyze: {error}");
            ExitCode::FAILURE
        }
    }
}
