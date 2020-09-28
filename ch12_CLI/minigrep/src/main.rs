use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfig = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for \'{}\'", cfig.query);
    println!("In file \'{}\'\n", cfig.filename);

    if let Err(e) = minigrep::run(cfig) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
