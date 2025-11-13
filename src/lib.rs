pub mod cli;

use cli::Cli;
use std::{
    error::Error,
    fs,
    io::{self, BufRead, BufReader, StdoutLock, Write},
};

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    for file in cli.files {
        print_file(&file, cli.show_line_numbers)?
    }

    Ok(())
}

fn print_file(file_path: &str, is_show_line_numbers: bool) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(file_path)?;

    let reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if is_show_line_numbers {
        show_line_with_number(reader, &mut handle)?;
    } else {
        show_line(reader, &mut handle)?;
    }

    Ok(())
}

fn show_line_with_number(
    reader: BufReader<fs::File>,
    handle: &mut StdoutLock,
) -> Result<(), Box<dyn Error>> {
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        writeln!(handle, "{:6}\t{}", index + 1, line)?
    }

    Ok(())
}

fn show_line(reader: BufReader<fs::File>, handle: &mut StdoutLock) -> Result<(), Box<dyn Error>> {
    for line in reader.lines() {
        let line = line?;
        writeln!(handle, "{}", line)?
    }

    Ok(())
}
