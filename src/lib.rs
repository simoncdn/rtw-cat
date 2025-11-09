pub mod config;

use std::{error::Error, fs, io::{self, BufReader}};

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(config.file_path)?;
    let mut reader = BufReader::new(file);
    let mut stdout = io::stdout();

    io::copy(&mut reader, &mut stdout)?;

    Ok(())
}
