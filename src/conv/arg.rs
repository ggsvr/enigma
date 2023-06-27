use super::Encoding;

#[derive(Debug, Clone)]
pub struct Args {
    pub input: Encoding,
    pub output: Encoding,
    pub msg: Vec<String>,
}

impl Args {
    pub fn get(mut args: std::env::Args) -> Result<Self, &'static str> {
        let input = args.next().ok_or("missing <input> argument")?;
        if input == "-h" || input == "--help" {
            return Err("help");
        }
        let output = args.next().ok_or("missing <output> argument")?;
        let msg: Vec<_> = args.collect();
        if msg.is_empty() {
            return Err("missing <msg> argument");
        }
        let input: Encoding = input.parse()?;
        let output: Encoding = output.parse()?;
        Ok(Self { input, output, msg })
    }
}
