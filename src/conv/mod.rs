pub mod arg;
pub mod exe;
pub use exe::run;

pub struct Num(u128);

#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    Ascii(Ascii),
    Hex(Hex),
    Oct(Oct),
    Bin(Bin),
    Dec(Dec),
}
impl Encoding {
    pub fn print(self, n: Num) -> String {
        match self {
            Self::Ascii(ascii) => ascii.print(n),
            Self::Hex(hex) => hex.print(n),
            Self::Oct(oct) => oct.print(n),
            Self::Bin(bin) => bin.print(n),
            Self::Dec(dec) => dec.print(n),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Ascii;
impl Ascii {
    pub fn parse(self, c: char) -> Result<Num, &'static str> {
        if !c.is_ascii() {
            return Err("character is not ASCII");
        }
        Ok(Num(c as u128))
    }
    pub fn print(self, Num(n): Num) -> String {
        n.to_be_bytes().iter().map(|b| *b as char).collect()
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Hex;
impl Hex {
    pub fn parse(self, num: &str) -> Result<Num, &'static str> {
        let mut chars = num.chars();
        if num.starts_with("0x") {
            chars.nth(1);
        }
        let mut num: u128 = 0;
        for c in chars {
            num = num
                .checked_mul(16)
                .ok_or("overflow reached. Too many characters")?;
            let nibble = c.to_digit(16).ok_or("character is not HEX")?;
            num += nibble as u128;
        }
        Ok(Num(num))
    }
    pub fn print(self, Num(n): Num) -> String {
        format!("{:x}", n)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Bin;
impl Bin {
    pub fn parse(self, num: &str) -> Result<Num, &'static str> {
        let mut chars = num.chars();
        if num.starts_with("0b") {
            chars.nth(1);
        }
        let mut num: u128 = 0;
        for c in chars {
            num = num
                .checked_shl(1)
                .ok_or("overflow reached. Too many characters")?;
            let bit = c.to_digit(2).ok_or("character is not BIN")?;
            num += bit as u128;
        }
        Ok(Num(num))
    }
    pub fn print(self, Num(n): Num) -> String {
        format!("{:b}", n)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Oct;
impl Oct {
    pub fn parse(self, num: &str) -> Result<Num, &'static str> {
        let mut chars = num.chars();
        if num.starts_with("0o") {
            chars.nth(1);
        }
        let mut num: u128 = 0;
        for c in chars {
            num = num
                .checked_mul(8)
                .ok_or("overflow reached. Too many characters")?;
            let octal = c.to_digit(8).ok_or("character is not OCT")?;
            num += octal as u128;
        }
        Ok(Num(num))
    }
    pub fn print(self, Num(n): Num) -> String {
        format!("{:o}", n)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Dec;
impl Dec {
    pub fn parse(self, num: &str) -> Result<Num, &'static str> {
        Ok(Num(num
            .parse::<u128>()
            .map_err(|_| "character is not DEC")?))
    }
    pub fn print(self, Num(n): Num) -> String {
        format!("{n}")
    }
}

impl std::str::FromStr for Encoding {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ascii" => Self::Ascii(Ascii),
            "hex" => Self::Hex(Hex),
            "oct" => Self::Oct(Oct),
            "bin" => Self::Bin(Bin),
            "dec" => Self::Dec(Dec),
            _ => return Err("invalid encoding"),
        })
    }
}
