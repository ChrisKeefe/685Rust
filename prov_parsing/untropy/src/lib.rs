use std::error::Error;
// TODO: someday
// use std::path::Path;
use std::fs::File;

#[derive(Debug)]
pub struct Config {
    pub fp: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // For now, we'll take in one fp only, Later, a Vec<String>
        if args.len() != 2 {
            return Err("Please provide exactly one fp argument");
        }

        let fp = args[1].clone();
        println!("Storing fp {} in config.", fp);

        Ok(Config { fp })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    println!("Now we have a config {:?}", conf);
    println!("Calling unzip on {}", conf.fp);
    let a_name = unzip_result(&conf.fp)?;
    println!("Something in the archive is named: {}", a_name);
    Ok(())
}

pub fn unzip_result(fp: &str) -> Result<&str, Box<dyn Error>> {
    println!("Unzipping {} ", fp);
    let fp = File::open(fp)?;
    // let reader = std::io::Cursor::new(fp);
    let mut zip = zip::ZipArchive::new(fp)?;
    for i in 0..zip.len(){
        println!("{}", zip.by_index(i).unwrap().name());
    }
    Ok(&"gerbils")
}