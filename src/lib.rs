use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
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
}