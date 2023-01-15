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
    let query = args[1].clone();
    let filename = args[2].clone();
    println!("Searching for {}", query);
    println!("In file {}", filename);
    let config = Config::new(&query, &filename);
    config
}

pub fn read_file(config: &Config) -> Result<String, io::Error> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("With text:\n{}", contents);
    Ok(contents)
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
    fn read_file_test() {
        let config = super::Config::new("needle", "./data/poem");
        let result = read_file(&config);
        assert!(result.is_ok());
    }
}