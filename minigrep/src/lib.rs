use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;
    println!("With text:\n{contents}");

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config { query, filepath })
    }
}

pub fn search_query<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut return_data = Vec::new();

    for line in content.lines() {
        if (line.contains(&query)) {
            return_data.push(line);
        }
    }
    return_data
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut return_data = Vec::new();

    for line in content.lines() {
        if (line.to_lowercase().contains(&query)) {
            return_data.push(line);
        }
    }
    return_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search() {
        let query = "duct";
        let content = "\
Rust:
safe,fast,productive
Pick three.";

        assert_eq!(vec!["safe,fast,productive"], search_query(query, content))
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
