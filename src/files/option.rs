use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "src/files/.options.txt";

/*
 * Read/write in a files all kebbterm opions.
 * Convert the speed value to speed tick used by Speed struct.
 */
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
    pub fn speed_conversion(&self) -> usize {
        match self.speed {
            1 => 600,
            2 => 500,
            3 => 400,
            4 => 300,
            5 => 200,
            6 => 100,
            7 => 75,
            8 => 50,
            9 => 25,
            _ => 0,
        }
    }

    pub fn new() -> Options {
        let mut options = Options {
            speed: 7,
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

            for (i, data) in txt.split('\n').enumerate() {
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

        writeln!(&mut f, "{}", self.speed)?;
        writeln!(&mut f, "{}", self.letter)?;
        writeln!(&mut f, "{}", self.capital)?;
        writeln!(&mut f, "{}", self.digit)?;
        writeln!(&mut f, "{}", self.symbol)?;
        writeln!(&mut f, "{}", self.french)?;
        writeln!(&mut f, "{}", self.french_cap)?;

        Ok(())
    }
}
