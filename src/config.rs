use std::{env, process};

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}

impl Config {
    fn new(file_path: String) -> Config {
        Config { file_path }
    }

    pub fn get_input_config() -> Config {
        let args: Vec<String> = env::args().collect();

        Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1)
        })
    }

    fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Usage: rtw-cat <path>");
        }

        let file_path = &args[1];

        Ok(Config::new(file_path.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        let config = Config::new(String::from("file.txt"));

        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_build() {
        let args = vec![String::from("program"), String::from("file.txt")];

        let config = Config::build(&args).unwrap();

        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn test_build_enough_argument() {
        let args = vec![String::from("program")];

        let result = Config::build(&args).is_err();

        assert!(result)
    }

    #[test]
    fn test_build_with_error() {
        let args = vec![String::from("program")];

        let result = Config::build(&args);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Usage: rtw-cat <path>".to_string())
    }
}
