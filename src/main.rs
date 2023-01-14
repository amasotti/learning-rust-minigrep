use std::env;
use std::fs;
use minigrep::{Config};

/// Main entry point for the mini grep cli tool
fn main() {
    let arguments : Vec<String> = env::args().collect();
    let config : Config = minigrep::parse_config(&arguments);
    minigrep::read_file(&config);

}

