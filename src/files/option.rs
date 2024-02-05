use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = ".options.txt";

#[derive(Debug)]
pub struct Options {
    pub speed: u8,
    pub alpha: bool,
    pub cap: bool,
    pub digit: bool,
    pub symbol: bool,
    pub french: bool,
}

impl Options {
    pub fn new() -> Options {
        Options {
            speed: 0,
            alpha: true,
            cap: true,
            digit: true,
            symbol: false,
            french: false,
        }
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        let mut file = File::open(FILE_NAME)?;

        let mut txt = String::new();
        file.read_to_string(&mut txt)?;

        for (i, data) in txt.split("\n").enumerate() {
            match i {
                0 => self.speed = data.parse::<u8>().unwrap_or(0),
                1 => self.alpha = data.parse::<bool>().unwrap_or(true),
                2 => self.cap = data.parse::<bool>().unwrap_or(true),
                3 => self.digit = data.parse::<bool>().unwrap_or(true),
                4 => self.symbol = data.parse::<bool>().unwrap_or(false),
                _ => self.french = data.parse::<bool>().unwrap_or(false),
            }
        }
        Ok(())
    }

    pub fn write(&self) -> std::io::Result<()> {
        // let mut f = File::options().write(true).truncate(true).open(FILE_NAME)?;
        let mut f = File::create(FILE_NAME)?;

        writeln!(&mut f, "{}", self.speed.to_string())?;
        writeln!(&mut f, "{}", self.alpha.to_string())?;
        writeln!(&mut f, "{}", self.cap.to_string())?;
        writeln!(&mut f, "{}", self.digit.to_string())?;
        writeln!(&mut f, "{}", self.symbol.to_string())?;
        writeln!(&mut f, "{}", self.french.to_string())?;

        Ok(())
    }
}
