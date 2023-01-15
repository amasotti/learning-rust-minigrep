use minigrep::Config;
use std::env;

/// Main entry point for the mini grep cli tool
fn main() {
    // Collect the arguments into a vector
    let arguments: Vec<String> = env::args().collect();

    // Create a new Config instance from the arguments
    let config: Config = Config::new_from_args(&arguments, true).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments; ERROR: {}", err);
        std::process::exit(1);
    });

    // Run the program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }

}
