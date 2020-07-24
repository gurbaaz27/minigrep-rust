extern crate clap;
use clap::{ArgMatches};

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive : String,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, &'static str> {

        let query = matches.value_of("query").expect("Hey! It was required").to_string();
        let filename = matches.value_of("filename").expect("Hey! It was required").to_string();
        let case_sensitive = matches.value_of("case_sensitive").expect("Hey! It was required").to_string();

        if case_sensitive != "y" && case_sensitive != "n" {
            return Err("type y or n for a valid argument!");
        }

        Ok(Config { query, 
                    filename,
                    case_sensitive,
         })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive == "y" {
    	search(&config.query, &contents)
    } else{
    	search_case_insensitive(&config.query, &contents)
    };

    for line in results {
    	println!("{}",line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
    	if line.contains(query){
    		results.push(line);
    	}
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
    	if line.to_lowercase().contains(&query){
    		results.push(line);
    	}
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
