// TODO: Can we drop std::error::Error in favor of std::io::Error, and lose the
// `as ioError`?
// NOTE: std::err::Error is a Trait, std::io::Error is a Type used for io errors
use std::env;
use std::error::Error;
use std::io::Error as ioError;
mod deserialization;
use deserialization::{build_tree, get_relevant_files, serialize_actions};

/// A Config struct to store command line arguments
/// NOTE: For simplicity, this should probably be a single filepath per analysis
/// If we allow users to pass multiple filepaths per analysis, determining how
/// to group them will be gross. 
/// TODO: Will "how do we handle CLI args?" be moot when we're exposing a WASM API?
#[derive(Debug)]
pub struct Config {
/// Not a Path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // ignore program name (CLI args[0])
        args.next();

        // extract filepath argument
        let fp = match args.next() {
            Some(arg) => arg,
            None => return Err("Please provide exactly one filepath argument"),
        };

        Ok(Config { fp })
    }
}

/// Main program logic
pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let relevant_files = get_relevant_files(&conf.fp)?;
    let root_id = &relevant_files.root_uuid.clone();
    let mut actions = serialize_actions(relevant_files)?;

    // TODO: make this a test instead
    // Confirm actions[0] is the root node.
    if actions[0].uuid.as_ref().unwrap() != root_id {
        return Err(Box::new(ioError::new(std::io::ErrorKind::InvalidInput,
            "serialize_actions error: returns non-root id at idx 0.")));
    }

    for i in 0..actions.len(){
        // println!("{:?}", actions[i].uuid);
        // println!("{:?}", actions[i].metadata);
        // println!("{:?}", actions[i].parents);
        // println!("");
    }

    let tree = build_tree(&mut actions)?;
    // println!("\n\nA horrible tree: {:#?}", tree);
    
    Ok(())
}

