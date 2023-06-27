use std::env;
use std::process::ExitCode;

static USAGE: &str = r#"USAGE: enigma <command> <args>
commands:
| conv: convert between different number representations
| morse: encode and decode morse code"#;
pub fn main() -> ExitCode {
    if let Err(e) = run() {
        if e == "help" {
            eprintln!("{USAGE}");
            return ExitCode::SUCCESS;
        } else {
            eprintln!("{e}\n");
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

pub fn run() -> Result<ExitCode, String> {
    let mut args = env::args();
    args.next();
    let command = args.next().ok_or("missing <command>")?;
    if command == "-h" || command == "--help" {
        return Err("help".into());
    }
    enigma::command(&command, args)
}
