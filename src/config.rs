use std::env;

pub struct Config {
    pub source: String,
    path: String,
}

impl Config {
    pub fn new() -> Config {
        let mut args = env::args();

        if args.len() != 2 {
            eprintln!("Usage: <source>");
            std::process::exit(1);
        }

        return Config {
            path: args.next().unwrap(),
            source: args.next().unwrap(),
        };
    }
}
