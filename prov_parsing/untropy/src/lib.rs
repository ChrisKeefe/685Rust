// TODO: remove
// #![allow(warnings)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;
// use std::io;
// use serde_yaml::Value;
// use serde_yaml::Result;

#[derive(Debug)]
pub struct Config {
    // Not path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    // No need to capture Execution or Environment details for now
    // serde gracefully drops missing keys by default.
    // execution: Execution,
    action: ActionDetails,
    // environment: serde_yaml::Value
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionDetails {
    #[serde(rename="type")]
    semantic_type: String,
    plugin: String,
    action: String,
    inputs: serde_yaml::Value,
    parameters: serde_yaml::Value,
    #[serde(rename="output-name")]
    output_name: String,
    // #[serde(rename="alias-of")]
    // alias_of: String,
    // params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionMetadata {
    uuid: String,
    #[serde(rename="type")]
    semantic_type: String,
    // This could probably be an Option(String), but we'll capture nulls as 
    // strings for now
    format: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Execution {
    uuid: String,
    runtime: serde_yaml::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProvNode {
    metadata: ActionMetadata,
    details: Action,
    children: Vec<ProvNode>,
}

#[derive(Debug)]
pub struct RelevantFiles {
    pub filenames: Vec<String>,
    pub contents: Vec<String>,
    // TODO: we need some kind of tuple/struct that describes 
    // metadata.yaml/action.yaml pairs. Do we even need filenames?
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
    let relevant_files = get_relevant_files(&conf.fp)?;
    println!("Things in the archive are named: ");
    // TODO: remove diagnostics
    // for i in 1..relevant_files.filenames.len() {
    //     println!("{}", relevant_files.filenames[i]);
    // }
    // println!("\nFirst archive contains: ");
    // println!("{}", relevant_files.contents[1]);

    let actions = serialize_actions(relevant_files)?;
    // let tree = build_tree(actions);

    Ok(())
}

pub fn serialize_actions(files: RelevantFiles) -> Result<Action, serde_yaml::Error> {
    let result: Action = serde_yaml::from_str(&files.contents[0])?;
    println!("{:?}", result);

    
    Ok(result)
}


// pub fn build_tree(actions: Vec<Action>) -> ProvNode {
//     // TODO: implementp)?;
//     let result = ProvNode::new();
//     result
// }

pub fn get_relevant_files(fp: &str) -> Result<RelevantFiles, Box<dyn Error>> {
    println!("Unzipping {} ", fp);
    // Get a filepath and create a ZipArchive
    let fp = File::open(fp)?;
    let mut zip = zip::ZipArchive::new(fp)?;

// TODO: Check the QIIME2 archive version, and handle appropriately.
// For now, that probably means error if version != 5

    // Create a positive mask for relevant files
    let filenames: Vec<String> = zip.file_names()
        .filter(|name| name.contains("provenance") 
                     & name.contains("action.yaml"))
        .map(|name| {String::from(name)})
        .collect();

    // Read files into memory
    let mut tmp_contents = String::new();
    let mut contents = Vec::new();
    for i in 0..filenames.len() {
        zip.by_name(&filenames[i]).unwrap().read_to_string(&mut tmp_contents).unwrap();
        contents.push(String::from(&tmp_contents));
    }

    Ok(RelevantFiles{ filenames, contents })
}