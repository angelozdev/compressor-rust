use std::env;

#[derive(Debug)]
pub struct Config {
    pub source: String,
    pub dir: String,
    pub path: String,
}

impl Config {
    pub fn new() -> Config {
        let mut args = env::args();

        if args.len() < 2 {
            eprintln!("Usage: compressor <source> [<dir>]");
            std::process::exit(1);
        }

        return Config {
            path: args.next().unwrap(),
            source: args.next().unwrap(),
            dir: args.next().unwrap_or(".".to_string()),
        };
    }
}
