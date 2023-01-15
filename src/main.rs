use minigrep::Config;
use std::env;

/// Main entry point for the mini grep cli tool
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config: Config = Config::new_from_args(&arguments, false).unwrap();
    let contents = match minigrep::read_file(&config) {
        Ok(c) => c,
        Err(e) => panic!("Error reading file: {}", e),
    };
    println!("With text:\n{}", contents);
}
