use std::{env, fs, process};

struct Config {
    paths: Vec<String>,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(format!("expected 1 argument, but {} were given", args.len() - 1));
        }
        Ok(Config { paths: args[1..].to_vec() })
    }
}

fn main() {
    let config = match Config::build(env::args().collect()) {
        Ok(config) => config,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    for path in config.paths {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };
        println!("{}", contents);
    };
}

