use std::{fs, io};

/// Simple struct to hold the configuration for the mini grep cli tool
///
/// # Properties
///
/// * `query` - The string to search for
/// * `filename` - The file to search in (as string filepath)
///
/// # Examples
///
/// ```
/// use minigrep::Config;
///
/// let config = Config {query: String::from("needle"), filename: String::from("bar.txt")};
/// ```
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// Create a new Config instance
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for
    /// * `filename` - The file to search in (as string filepath)
    ///
    /// # Examples
    ///
    /// ```
    /// use minigrep::Config;
    ///
    /// let config = Config::new("needle", "bar.txt");
    /// assert_eq!(config.query, "needle");
    /// assert_eq!(config.filename, "bar.txt");
    ///
    /// ```
    pub fn new(query: &str, filename: &str) -> Config {
        Config {
            query: query.to_string(),
            filename: filename.to_string(),
        }
    }

    /// Create a new Config instance from a vector of arguments
    /// The first argument is ignored, as it is the program name
    /// The second argument is the **query** string
    /// The third argument is the **filename**
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of arguments
    ///
    /// # Returns
    ///
    /// A new [Config] instance
    ///
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use minigrep::Config;
    ///
    /// let args = vec![String::from("program_name"), String::from("needle"), String::from("bar.txt")];
    /// let config = Config::new_from_args(&args,false).unwrap();
    /// assert_eq!(config.query, "needle");
    /// assert_eq!(config.filename, "bar.txt");
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// If the number of arguments is not 3
    ///
    pub fn new_from_args(args: &[String], dbg: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. USAGE is: minigrep <query> <filename>");
        }

        if dbg {
            println!("args: {:?}", args);
            println!("Searching for {}", &args[1].clone());
            println!("In file {}", &args[2].clone());
        }

        Ok(Config::new(&args[1].clone(), &args[2].clone()))
    }
}
/// Parse the command line arguments into a Config struct
///
/// # Arguments
///
/// * `args` - A vector of command line arguments
///
/// # Returns
///
/// A [Config] struct
///
///
pub fn parse_config(args: &[String]) -> Config {
    let config = Config::new_from_args(&args, true);
    match config {
        Ok(c) => c,
        Err(e) => panic!("Error while parsing the cli arguments: {}", e),
    }
}

pub fn read_file(config: &Config) -> Result<String, io::Error> {
    let contents = fs::read_to_string(&config.filename)?;
    Ok(contents)
}


pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file content
    let contents = read_file(&config)?;
    //println!("With text:\n{}", contents);

    let mut counter = 1;
    for line in search(&config.query, &contents) {
        println!("Finding #{}: {}", counter, line);
        counter += 1;
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in contents.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_test() {
        let test_args = vec!["".to_string(), "query".to_string(), "filename".to_string()];
        let config = parse_config(&test_args);
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    #[should_panic]
    fn parse_config_with_error() {
        let test_args = vec!["query".to_string(), "filename".to_string()];
        parse_config(&test_args);
    }

    #[test]
    fn read_file_test() {
        let config = super::Config::new("needle", "./data/poem");
        let result = read_file(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_config_with_too_few_args() {
        let test_args = vec!["".to_string(), "query".to_string()];
        let config = Config::new_from_args(&test_args, false);
        assert!(config.is_err());
    }

    #[test]
    #[should_panic]
    fn parse_config_panic_test() {
        let test_args = vec!["".to_string(), "query".to_string()];
        let config = Config::new_from_args(&test_args, false);
        if config.is_err() {
            panic!("Not enough arguments");
        }
    }

    #[test]
    fn run_test() {
        let query = "needle";
        let content = "\
Rust:
safe, fast, productive.
needle in the haystack
Pick three.";
        assert_eq!(vec!["needle in the haystack"], search(query, content));
    }
}
