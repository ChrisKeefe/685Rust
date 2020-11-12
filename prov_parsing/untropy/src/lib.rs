use std::error::Error;
// use std::path::{Path, PathBuf};
// use std::fs;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::io;
// use serde_json::Value::*;
// use serde_json::Result;

#[derive(Debug)]
pub struct Config {
    // Not path b/c we don't need anything special from the root filepath.
    pub fp: String,
}

// TODO: This will probably become serde_json
#[derive(Debug)]
pub struct RelevantFiles {
    pub contents: Vec<String>,
}

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
    let action_fps = get_relevant_files(&conf.fp)?;
    // println!("Things in the archive are named: ");
    // for i in 1..action_fps.filenames.len() {
    //     println!("{}", (*action_fps.filenames[i]).display());
    // }

    // let actions = scrape_actions(action_fps)?;
    // let tree = build_tree(actions);

    Ok(())
}

// pub fn scrape_actions(mut action_filenames: RelevantFiles)
//          -> Result<Vec<Action>, Box<dyn Error>> {
//     // get one filepath
//     let first_fp = &(action_filenames.filenames.pop().unwrap());
//     println!("First fp: {}", first_fp.display());
    
//     // unzip and read
//     // let fp = File::open(&first_fp)?;
//     // let mut zip = zip::ZipArchive::new(&fp)?;
//     // let file = zip.by_name(first_fp.to_str().unwrap())?;
//     // let contents = std::fs::read(file)?;
//     // println!("{}", contents[0]);




//     // let mut action = HashMap::new();
//     // action.insert("Gerbil".to_string(),
//     //               "Guts".to_string());
//     // println!("{:?}", action);

//     Ok(vec![Action::new()])
// }


pub fn build_tree(actions: Vec<Action>) -> ProvNode {
    // TODO: implement
    let result = ProvNode::new();
    result
}

// TODO: if we have multiple filters, we could make this generic, and pass them
// in go get whatever subset of results we want
pub fn get_relevant_files(fp: &str) -> Result<RelevantFiles, Box<dyn Error>> {
    println!("Unzipping {} ", fp);
    // Get a filepath and create a ZipArchive
    let fp = File::open(fp)?;
    let mut zip = zip::ZipArchive::new(fp)?;

    // Create a mask for relevant files
    // let filenames = zip.file_names()
    //     .filter(|name| name.contains("provenance") & name.contains("action.yaml"));


    // Read files into memory
    let mut tmp_contents = String::new();
    let mut contents = Vec::new();
    for i in 0..zip.len() {
        zip.by_index(i).unwrap().read_to_string(&mut tmp_contents);
        contents.push(String::from(&tmp_contents));
    }

    // let remaining_filenames = filenames
    //     .map(|name| {String::from(name)})
    //     .collect();

    // TODO: remove - visual test for valid read
    println!("{:?}", contents[0]);

    Ok(RelevantFiles{ contents })
}