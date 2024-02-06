use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "src/files/.options.txt";

#[derive(Debug)]
pub struct Options {
    pub speed: u8,
    pub letter: bool,
    pub capital: bool,
    pub digit: bool,
    pub symbol: bool,
    pub french: bool,
    pub french_cap: bool,
}

impl Options {
    pub fn new() -> Options {
        let mut options = Options {
            speed: 0,
            letter: true,
            capital: true,
            digit: true,
            symbol: false,
            french: false,
            french_cap: false,
        };

        if let Ok(mut file) = File::open(FILE_NAME) {
            let mut txt = String::new();
            let _ = file.read_to_string(&mut txt);

            for (i, data) in txt.split("\n").enumerate() {
                match i {
                    0 => options.speed = data.parse::<u8>().unwrap_or(0),
                    1 => options.letter = data.parse::<bool>().unwrap_or(true),
                    2 => options.capital = data.parse::<bool>().unwrap_or(true),
                    3 => options.digit = data.parse::<bool>().unwrap_or(true),
                    4 => options.symbol = data.parse::<bool>().unwrap_or(false),
                    5 => options.french = data.parse::<bool>().unwrap_or(false),
                    6 => options.french_cap = data.parse::<bool>().unwrap_or(false),
                    _ => {}
                }
            }
        };

        options
    }

    pub fn write(&self) -> std::io::Result<()> {
        // let mut f = File::options().write(true).truncate(true).open(FILE_NAME)?;
        let mut f = File::create(FILE_NAME)?;

        writeln!(&mut f, "{}", self.speed.to_string())?;
        writeln!(&mut f, "{}", self.letter.to_string())?;
        writeln!(&mut f, "{}", self.capital.to_string())?;
        writeln!(&mut f, "{}", self.digit.to_string())?;
        writeln!(&mut f, "{}", self.symbol.to_string())?;
        writeln!(&mut f, "{}", self.french.to_string())?;
        writeln!(&mut f, "{}", self.french_cap.to_string())?;

        Ok(())
    }
}
