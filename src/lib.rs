pub mod conv;
pub mod morse;
use phf::phf_map;
use std::env::Args;
use std::process::ExitCode;

static COMMANDS: phf::Map<&'static str, fn(Args) -> ExitCode> = phf_map! {
    "conv" => conv::run,
    "morse" => morse::run,
};

pub fn command(command: &str, args: Args) -> Result<ExitCode, String> {
    match COMMANDS.get(command) {
        Some(f) => Ok(f(args)),
        None => Err(format!("command `{command}` does not exist")),
    }
}
