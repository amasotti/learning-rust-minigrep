use std::{fs, io};

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    Config { query: query.to_string(), filename: filename.to_string() }
}

pub fn read_file(config: &Config) -> Result<String, io::Error> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("With text:\n{}", contents);
    Ok(contents)
}