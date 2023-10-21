use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    opt: Option<String>,
    arg: String,
    out_set: bool,
    out_file: Option<String>,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, String> {
        match args.len() {
            2 => {
                return Ok(
                    Config {
                        opt: None, 
                        arg: args[1].clone(),
                        out_set: false,
                        out_file: None,
                    }
                );
            },
            3 => {
                if is_option(&args[1]) {
                    return Ok(
                        Config { 
                            opt: Some(args[1].clone()), 
                            arg: args[2].clone(),
                            out_set: false,
                            out_file: None,
                        }
                    );
                }
                return Err(format!("{} is not an option", args[1]));
            },
            5 | 4 => {
                if is_option(&args[1]) && args.len() == 5 {
                    if args[args.len() - 2] == ">" {
                        return Ok(
                            Config { 
                                opt: Some(args[1].clone()), 
                                arg: args[2].clone(),
                                out_set: true,
                                out_file: Some(args[args.len() - 2].clone()),
                            }
                        );
                    }
                    return Err(String::from("invalid output pattern"));
                }
                return Err(format!("{} is not an option", args[1]));
            },
            _ => {
                return Err(format!("command `echo` takes 1, 2 or 4 arguments, but {} were given", args.len() - 1));
            },
        } 
    }

    pub fn run(self) -> Result<String, String> {
        match self.opt {
            Some(opt) => {
                if opt == "-e" {
                    return Ok(self.arg.replace("\\n", "\n").replace("\\r", "\r").replace("\\t", "\t"));
                }
            },
            None => {
                if let Ok(value) = env::var(&self.arg) {
                    return Ok(value);
                };
                if self.out_set {
                    match fs::write(PathBuf::from(self.out_file.unwrap()), self.arg) {
                        Ok(_) => {
                            return Ok(String::from("succesfully written to file"));
                        },
                        Err(e) => {
                            return Err(format!("{}", e));
                        },
                    }
                }
            }
        }
        Ok(self.arg)
    }
}

fn is_option(arg: &String) -> bool {
    arg.starts_with("-")
}