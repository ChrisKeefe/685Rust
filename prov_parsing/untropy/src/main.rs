use std::env;
use std::process;

#[derive(Debug)]
struct Config {
    fp: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // For now, we'll take in one fp only, Later, a Vec<String>
        if args.len() != 2 {
            return Err("Please provide exactly one fp argument");
        }

        // TODO: factor out clone for performance? (book Ch13)
        let fp = args[1].clone();
        println!("Storing fp {} in config.", fp);

        Ok(Config { fp })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Invalid arguments: {}", err);
        process::exit(1);
    });

    run(config);
    process::exit(0);
}


fn run(conf: Config) {
    println!("Now we have a config {:?}", conf);
}
