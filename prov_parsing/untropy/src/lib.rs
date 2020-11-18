// TODO: Can we drop std::error::Error in favor of std::io::Error, and lose the
// `as ioError`?
use std::error::Error;
use std::io::Error as ioError;
mod deserialization;
use deserialization::{build_tree, get_relevant_files, serialize_actions};

/// A Config struct to store command line arguments
#[derive(Debug)]
pub struct Config {
/// Not a Path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // For now, we'll take in one fp only, Later, a Vec<String>
        if args.len() != 2 {
            return Err("Please provide exactly one fp argument");
        }

        let fp = args[1].clone();
        Ok(Config { fp })
    }
}

/// Main run function for the program - primary program logic lives here
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
        println!("{:?}", actions[i].uuid);
        // println!("{:?}\n", actions[i].metadata);
        // println!("{:?}\n", actions[i].children);
        println!("");
    }

    let tree = build_tree(&mut actions)?;
    println!("A horrible tree: {:#?}", tree);
    
    Ok(())
}

