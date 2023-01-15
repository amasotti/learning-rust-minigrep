use minigrep::Config;
use std::env;

/// Main entry point for the mini grep cli tool
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config: Config = Config::new_from_args(&arguments, false).unwrap();
    minigrep::read_file(&config);
}
