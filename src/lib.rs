use std::env;
use std::error::Error;
use std::fs;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let search_string = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(search_string, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let search_string = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(search_string, contents)
        );
    }
}

#[derive(Debug)]
pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    fn new(search_string: String, file_path: String, ignore_case: bool) -> Config {
        Config {
            search_string: search_string,
            file_path: file_path,
            ignore_case: ignore_case,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    if config.ignore_case {
        for result in search_case_insensitive(&config.search_string, &contents) {
            println!("{}", result);
        }
    } else {
        for result in search(&config.search_string, &contents) {
            println!("{}", result);
        }
    }

    Ok(())
}

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let search_string = match args.next() {
        Some(v) => v,
        None => return Err("no enough args"),
    };

    let file_path = match args.next() {
        Some(v) => v,
        None => return Err("no enough args"),
    };
    let ignore_case = match env::var("IGNORE_CASE") {
        Ok(t) => {
            if t == "1" {
                true
            } else {
                false
            }
        }
        Err(e) => false,
    };
    let config = Config::new(search_string, file_path, ignore_case);
    Ok(config)
}

pub fn search<'a>(search_string: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive(search_string: &str, contents: &str) -> Vec<String> {
    let low_case_search_string = search_string.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&low_case_search_string) {
            results.push(line.to_string());
        }
    }
    results
}
