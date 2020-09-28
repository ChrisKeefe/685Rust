use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
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
        
        Ok(Config {query, filename})
    }
}

pub fn run(cfig: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfig.filename)?;
    
    for line in search(&cfig.query, &contents){
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let result = search(query, contents);
        println!("{:?}", result);
        assert_eq!(vec!["safe, fast, productive."], result);
    }

}