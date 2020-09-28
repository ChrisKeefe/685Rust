use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str>{
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfig = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for \'{}\'", cfig.query);
    println!("In file \'{}\'\n", cfig.filename);

    if let Err(e) = run(cfig) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(cfig: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfig.filename)?;
    
    println!("With text:\n{}", contents);

    Ok(())
}