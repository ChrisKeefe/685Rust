use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::File;

#[derive(Debug)]
pub struct Config {
    // Not path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

// TODO: This will probably become serde_json
#[derive(Debug)]
pub struct RelevantFiles {
    pub filenames: Vec<Box<Path>>,
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
    let names = get_relevant_paths(&conf.fp)?;
    println!("Things in the archive are named: ");
    for i in 1..names.filenames.len() {
        println!("{}", (*names.filenames[i]).display());
    }
    Ok(())
}

pub fn get_relevant_paths(fp: &str) -> Result<RelevantFiles, Box<dyn Error>> {
    println!("Unzipping {} ", fp);
    // Get a filepath so we can create a ZipArchive
    let fp = File::open(fp)?;
    let zip = zip::ZipArchive::new(fp)?;

    // Remove all non-provenance paths and box it up
    let filenames = zip.file_names()
        .filter(|name| name.contains("provenance"))
        .map(|name| {PathBuf::from(name).into_boxed_path()})
        .collect();

    Ok(RelevantFiles{ filenames })
}