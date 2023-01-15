use std::{env, fs, io};

/// Simple struct to hold the configuration for the mini grep cli tool
///
/// # Properties
///
/// * `query` - The string to search for
/// * `filename` - The file to search in (as string filepath)
///
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    /// Create a new Config instance
    ///
    /// # Arguments
    ///
    /// * `query` - The string to search for
    /// * `filename` - The file to search in (as string filepath)
    /// * `ignore_case` - Whether to ignore case when searching
    ///
    /// The `ignore_case` argument reads the `MINIGREP_IGNORE_CASE` environment variable
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
            ignore_case: env::var("MINIGREP_IGNORE_CASE").is_ok()
                && env::var("MINIGREP_IGNORE_CASE").unwrap() == "1",
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
            println!("Searching for {}", &args[1].clone());
            println!("In file {}", &args[2].clone());
        }

        Ok(Config::new(&args[1].clone(), &args[2].clone()))
    }
}

/// SearchResult is a simple struct to hold the results of a search
/// It contains the line number and the line itself
///
/// # Properties
///
/// * `line_number` - The line number
/// * `line` - The line itself
///
/// # Examples
///
/// ```
/// use minigrep::SearchResult;
///
/// let result = SearchResult::new(String::from("Hello world!"), 1);
/// assert_eq!(result.line_number, 1);
///
/// ```
pub struct SearchResult {
    pub line: String,
    pub line_number: usize,
}

impl SearchResult {
    /// Create a new SearchResult instance
    /// # Arguments
    ///
    /// * `line` - The line that was found
    /// * `line_number` - The line number of the line that was found
    ///
    pub fn new(line: String, line_number: usize) -> SearchResult {
        SearchResult { line, line_number }
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

/// `read_file` reads the contents of a file into a string
///
/// # Arguments
///
/// * `filename` - The file to read
///
/// # Returns
///
/// A string containing the contents of the file or an error
///
pub fn read_file(config: &Config) -> Result<String, io::Error> {
    let contents = fs::read_to_string(&config.filename)?;
    Ok(contents)
}

/// `run` is the main entry point for the mini grep cli tool
/// given a [Config] created before, it will read the file and search for the query string
///
/// # Arguments
///
/// * `config` - A [Config] struct
///
/// # Returns
///
/// A [Result] containing a vector of [SearchResult] or an error.
/// The search function is agnostic about the type of Error, it uses [Box] with a dynamic
/// dispatch to handle the error
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file content
    let contents = read_file(&config)?;
    //println!("With text:\n{}", contents);

    let mut counter = 1;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for result in results {
        println!(
            "Finding #{} at line {} :: {}",
            counter, &result.line_number, &result.line
        );
        counter += 1;
    }

    Ok(())
}

/// `search` searches for a query string in a string
/// It returns a vector of [SearchResult]
///
/// # Arguments
///
/// * `query` - The string to search for
/// * `contents` - The string to search in
///
/// # Returns
///
/// A vector of [SearchResult]
pub fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            let result = SearchResult::new(line.to_string(), i + 1);
            results.push(result);
        }
    }

    if results.len() == 0 {
        println!("---> No results found");
    }

    results
}

/// `search_case_insensitive` searches for a query string in a string. It's very similar
/// to [search] but it ignores the case of the query and the contents
///
/// # Arguments
///
/// * `query` - The string to search for
/// * `contents` - The string to search in
///
/// # Returns
///
/// A vector of [SearchResult]
pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results: Vec<SearchResult> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            let result = SearchResult::new(line.to_string(), i + 1);
            results.push(result);
        }
    }

    if results.len() == 0 {
        println!("---> No results found");
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
        let result = search(query, content);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].line, "needle in the haystack");
        assert_eq!(result[0].line_number, 3);
    }

    #[test]
    fn run_test_no_results() {
        let query = "needle";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        let result = search(query, content);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn run_case_insensitive_search() {
        let query = "NEEDLE";
        let content = "\
Rust:
safe, fast, productive.
needle in the haystack
Pick three.";
        let result = search_case_insensitive(query, content);
        assert_eq!(result.len(), 1, "Expected 1 result, got {}", result.len());
        assert_eq!(
            result[0].line, "needle in the haystack",
            "Expected 'needle in the haystack', got {}",
            result[0].line
        );
        assert_eq!(
            result[0].line_number, 3,
            "Expected line number 3, got {}",
            result[0].line_number
        );
    }
}
