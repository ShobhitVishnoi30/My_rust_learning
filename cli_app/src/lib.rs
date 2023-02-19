use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    let results;
    if config.ignore_case {
        results = search_case_sensitive(&config.query, &contents);
    } else {
        results = search(&config.query, &contents);
    };

    println!("Result {:?}", results);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient argument");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_name,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query = "duct";
        let content = "\
Rust:
safe,fast,productive
Pick three.";

        assert_eq!(vec!["safe,fast,productive"], search(query, content));
    }

    #[test]
    fn case_sensitive_search_test() {
        let query = "Hello";
        let content = "\
Rust:
Hello
safe,fast,productive
Pick three.
hello";

        assert_eq!(
            vec!["Hello", "hello"],
            search_case_sensitive(query, content)
        );
    }
}
