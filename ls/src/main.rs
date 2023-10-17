use std::{env, fs, process};

struct Config {
    path: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, String> {
        if args.len() != 2 {
            return Err(format!("expected 1 argument, but {} were given", args.len() - 1));
        }
        Ok(Config { path: args[1].clone() })
    }
}

fn main() {
    let config = match Config::build(env::args().collect()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let dir = match fs::read_dir(config.path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    for df in dir {
        println!("{:?}", df.unwrap().file_name());
    }
}