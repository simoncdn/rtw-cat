use clap::Parser;

#[derive(Parser)]
#[command(name = "rtw-cat")]
#[command(version = "0.1.0")]
#[command(about = "A cat clone written in Rust")]
pub struct Cli {
    /// The file to display
    pub files: Vec<String>,

    /// Show line numbers
    #[arg(short = 'n', long = "number")]
    pub show_line_numbers: bool,
}
