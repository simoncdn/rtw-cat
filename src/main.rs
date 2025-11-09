use std::process;

use rtw_cat::{config, run};

fn main() {
    let config = config::Config::get_input_config();

    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        process::exit(1)
    };
}
