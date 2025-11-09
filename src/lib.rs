pub mod config;

use std::{error::Error, fs};

use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_path)?;

    println!("{}", file);

    Ok(())
}
