use rtw_cat::{config, run};

fn main() {
    let config = config::Config::get_input_config();
    println!("{}", config.file_path);
    run();
}
