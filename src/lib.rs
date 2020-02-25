use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let search_result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search_result {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let lowercase_query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("Didn't get a query string")
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("Didn't get a filename string")
        };
        
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn less_than_3_arguments_provided() {
        // arrange
        let args: [String; 2] = [String::from("first"), String::from("second")];

        // act & assert
        Config::new(&args).unwrap();
    }

    #[test]
    fn can_create() {
        // arrange
        let args: [String; 3] = [String::default(), String::default(), String::default()];

        // act
        let result = Config::new(&args);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn created_config_has_correct_properties() {
        // arrange
        let args: [String; 3] = [String::from("ignore"), String::from("someQuery"), String::from("someFile.txt")];
        let query = &args[1];
        let filename = &args[2];
        
        // act 
        let result = Config::new(&args).unwrap();

        // assert
        assert_eq!(query, &result.query);
        assert_eq!(filename, &result.filename);
    }

    #[test]
    fn case_sensitive() {
        // arrange
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Duct tape.";
        
        // act
        let result = search(query, contents);

        // assert
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn case_insensitive() {
        // arrange
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.
Trust me.";

        // act
        let result = search_case_insensitive(&query, &contents);

        // assert
        assert_eq!(vec!["Rust:", "Trust me."], result)
    }
}