use std::process::ExitCode;
static HELP: &str = r#"USAGE: morse <message>

The program will detect if it needs
to encode or decode the message.
"#;

pub fn run(args: std::env::Args) -> ExitCode {
    let mut msg = String::new();
    for arg in args {
        msg.push_str(&arg);
        msg.push(' ');
    }
    if msg.is_empty() {
        return error("missing <message>");
    };
    if msg == "-h" || msg == "--help" {
        eprintln!("{HELP}");
        return ExitCode::SUCCESS;
    }
    let first = msg.chars().next().unwrap_or('-');
    let output = match first {
        '.' | '-' => super::decode(&msg),
        _ => super::encode(&msg),
    };
    let Some(output) = output else {
        return error("invalid characters found");
    };
    println!("{output}");
    ExitCode::SUCCESS
}

fn error(msg: &str) -> ExitCode {
    eprintln!("error: {msg}");
    eprintln!("{HELP}");
    ExitCode::FAILURE
}
