use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        // NOTE: we disregard args[0], which holds the executable's filepath
        // Here, we treat only args 1 and 2 as args for communication purposes
        let argslen = args.len() - 1;
        if argslen != 2{
            return Err("Exactly two command-line arguments required.");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(cfig: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfig.filename)?;
    
    let results = if cfig.case_sensitive {
        search(&cfig.query, &contents)
    } else {
        search_case_insensitive(&cfig.query, &contents)
    };

    for line in results {
        println!("{}", line);
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

pub fn search_case_insensitive<'a>(query: &str,
                                   contents: &'a str) -> Vec<&'a str> {
    let lc_query = query.to_lowercase();
    let mut results = Vec::new();


    for line in contents.lines(){
        if line.to_lowercase().contains(&lc_query){
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

        let result = search(query, contents);
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let result = search_case_insensitive(query, contents);
        assert_eq!(vec!["Rust:", "Trust me."], result);
    }
}