use std::env;
use std::fs;
/// Main entry point for the mini grep cli tool
fn main() {
    let arguments : Vec<String> = env::args().collect();

    let query = &arguments[1];
    let filename = &arguments[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);


    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

