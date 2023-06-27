use super::arg::Args;
use super::*;
use std::env;
use std::process::ExitCode;
static USAGE: &str = r#"USAGE: conv <input> <output> <msg>
encodings:
| bin
| oct
| dec
| hex
| ascii"#;
pub fn run(args: env::Args) -> ExitCode {
    if let Err(e) = _run(args) {
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
fn _run(args: env::Args) -> Result<(), &'static str> {
    let Args { input, output, msg } = Args::get(args)?;
    for word in msg {
        match input {
            Encoding::Ascii(ascii) => {
                for c in word.chars() {
                    let n = ascii.parse(c)?;
                    print!("{} ", output.print(n));
                }
            }
            Encoding::Bin(bin) => {
                let n = bin.parse(&word)?;
                print!("{} ", output.print(n));
            }
            Encoding::Dec(dec) => {
                let n = dec.parse(&word)?;
                print!("{} ", output.print(n));
            }
            Encoding::Hex(hex) => {
                let n = hex.parse(&word)?;
                print!("{} ", output.print(n));
            }
            Encoding::Oct(oct) => {
                let n = oct.parse(&word)?;
                print!("{} ", output.print(n));
            }
        }
    }
    println!();
    Ok(())
}
