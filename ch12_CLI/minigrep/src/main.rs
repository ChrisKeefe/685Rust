use std::env;
use std::fs;

struct Config {
    filename: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Config{
        // NOTE: we disregard args[0], which holds the executable's filepath

        let query = args[1].clone();
        let filename = args[2].clone();
        
        Config {query, filename}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfig = Config::new(&args);
    println!("Searching for \'{}\'", cfig.query);
    
    read_file(&cfig.filename);
    // TODO: improve error handling
    // TODO: consolidate all error handling in one place
    let contents = fs::read_to_string(cfig.filename)
    .expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
}


fn read_file(filename: &str) -> String {
    println!("In file \'{}\'\n", filename);
    String::from("remove this")
}