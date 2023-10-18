use std::env;

#[derive(Debug)]
pub struct Config {
    opt: Option<String>,
    arg: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err(format!("command `echo` takes 1 argument, but {} were given", args.len() - 1));
        } else if args.len() == 2 {
            return Ok(
                Config {
                    opt: None, 
                    arg: args[1].clone()
                }
            );
        } else if args.len() == 3 && args[1].starts_with("-") {
            return Ok(
                Config { 
                    opt: Some(args[1].clone()), 
                    arg: args[2].clone()
                }
            );
        } 
        return Err(format!("`{}` is not an option", args[1]));
    }

    pub fn run(self) -> String {
        match self.opt {
            Some(opt) => {
                if opt == "-e" {
                    return self.arg.replace("\\n", "\n").replace("\\r", "\r").replace("\\t", "\t");
                }
            },
            None => {
                if let Ok(value) = env::var(&self.arg) {
                    return value;
                };
            }
        }
        self.arg
    }
}