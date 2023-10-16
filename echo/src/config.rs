use std::env;

pub struct Config {
    arg: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, String> {
        if args.len() > 2 || args.len() < 2{
            return Err(format!("command `echo` takes 1 argument, but {} were given", args.len() - 1));
        }
        return Ok(Config {arg: args[1].clone()});
    }

    pub fn run(self) -> String {
        match env::var(&self.arg) {
            Ok(value) => return value,
            Err(_) => (),
        }
        self.arg
    }
}