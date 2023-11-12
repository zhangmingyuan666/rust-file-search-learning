// in main.rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);

    println!("searching for query: {}", config.query);
    println!("filePath: {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Config { query, file_path }
    }
}

// cargo run -- the poem.txt
