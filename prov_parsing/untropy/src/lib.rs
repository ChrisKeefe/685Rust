// TODO: remove
// #![allow(warnings)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fs;
// use std::io;
// use serde_yaml::Value;
// use serde_yaml::Result;

#[derive(Debug)]
pub struct Config {
    // Not path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

#[derive(Debug)]
pub struct RelevantFiles {
    pub filenames: Vec<String>,
    pub contents: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dummy {
    yaml: serde_yaml::Value
}

// TODO: This will probably become serde_yaml
#[derive(Debug, Deserialize, Serialize)]
pub struct ProvNode {
    uuid: String,
    sem_type: String,
    archive: u8,
    framework: String,
    format: String,
    children: Vec<ProvNode>,
}

impl ProvNode {
    pub fn new() -> ProvNode {
        ProvNode {
            uuid: String::from(""),
            sem_type: String::from(""),
            // TODO: Deal with archive versions
            archive: 5,
            framework: String::from(""),
            format: String::from(""),
            children: Vec::new(),   
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    plugin: String,
    action_name: String,
    params: HashMap<String, String>,
}

impl Action {
    pub fn new() -> Action {
            Action{
            plugin: String::from(""),
            action_name: String::from(""),
            params: HashMap::new(),
        }
    }
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

pub fn serialize_actions(files: RelevantFiles) -> Result<Dummy, serde_yaml::Error> {
    let result = serde_yaml::from_str(&files.contents[0])?;
    println!("SOME_YAML: {:?}", result);

    
    Ok(Dummy{yaml: result})
}


// pub fn build_tree(actions: Vec<Action>) -> ProvNode {
//     // TODO: implement
//     let result = ProvNode::new();
//     result
// }

pub fn get_relevant_files(fp: &str) -> Result<RelevantFiles, Box<dyn Error>> {
    println!("Unzipping {} ", fp);
    // Get a filepath and create a ZipArchive
    let fp = File::open(fp)?;
    let mut zip = zip::ZipArchive::new(fp)?;

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