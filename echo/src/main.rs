use std::{env, process};

use echo::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::build(args) {
        Ok(config) => config,
        Err(e) => {eprintln!("ERROR: {}", e); process::exit(1)},
    };
    println!("{}", config.run());
}
