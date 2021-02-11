use std::env;
use std::process;
use untropy::Config;

// TODO: why do the two blocks in main use two different error reporting strategies?
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Invalid arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = untropy::run(config){
        println!("Runtime Error: {}", e);
        process::exit(1);
    };

    process::exit(0);
}
