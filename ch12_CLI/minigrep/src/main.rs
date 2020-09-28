use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfig = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(cfig) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
