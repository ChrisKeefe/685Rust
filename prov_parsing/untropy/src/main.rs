use std::env;
use std::process;

#[derive(Debug)]
struct Config {
    fp: String,
}

// Build a Config constructor

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    run(config);
    process::exit(0);
}


fn parse_config(args: &[String]) -> Config {
    // For now, we'll take in one fp only
    // Later, a Vec<String>
    // TODO: Use a Result here: Ch12.03
    if args.len() != 2 {
        println!("Please provide exactly one fp argument");
        process::exit(1);
    }

    // TODO: factor out clone for performance (book Ch13)
    let fp = args[1].clone();
    println!("Storing fp {} in config.", fp);

    Config { fp }
}

fn run(conf: Config) {
    println!("Now we have a config {:?}", conf);
}
