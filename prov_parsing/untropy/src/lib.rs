// TODO: remove
// #![allow(warnings)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::path::PathBuf;

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
        println!("Storing fp {} in config.", fp);

        Ok(Config { fp })
    }
}

/// Select contents of an action.yaml file
#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    action: ActionDetails,
    // No need to capture the details in Execution or Environment objects for now
    // serde gracefully drops missing keys by default.
}

/// Data from the action tag in an action.yaml
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
    // TODO: what even is alias-of?
    // #[serde(rename="alias-of")]
    // alias_of: String,
    // params: HashMap<String, String>,
}

/// Contents of a metadata.yaml file
#[derive(Debug, Deserialize, Serialize)]
pub struct ActionMetadata {
    uuid: String,
    #[serde(rename="type")]
    semantic_type: String,
    // We'll capture nulls as Strings instead of Option(String)s for simplicity
    format: String,
}

/// One node of a provenance tree
#[derive(Debug, Deserialize, Serialize)]
pub struct ProvNode {
    metadata: Option<ActionMetadata>,
    action: Option<Action>,
    citations: Option<String>,
    children: Option<Vec<ProvNode>>,
}

impl ProvNode {
    pub fn new(filenames: Vec<String>, rel_files: RelevantFiles) 
            -> Result<ProvNode, serde_yaml::Error> {
        let mut metadata: Option<ActionMetadata> = None;
        let mut action: Option<Action> = None;
        let mut citations = None;
        let key_err = "Key Error in ProvNode::new(); Filepath does not exist in RelevantFiles";
        for i in filenames {
            let content = rel_files.0.get(&i).ok_or_else(|| {key_err});
            if i.contains("metadata.yaml") {
                metadata = serde_yaml::from_str(content.unwrap())?;
            } else if i.contains("action.yaml") {
                action = serde_yaml::from_str(content.unwrap())?;
            } else if i.contains("citations.bib") {
                citations = Some(String::from(content.unwrap()));
            }
        }

        Ok(ProvNode { metadata, action, citations, children: None })
    }
}

/// A HashMap of filename:content pairs
#[derive(Debug)]
pub struct RelevantFiles ( HashMap<String, String> );

impl RelevantFiles {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn new() -> RelevantFiles {
        let val = HashMap::new();
        RelevantFiles ( val )
    }

    pub fn insert(&mut self, filename: String, content: String) {
        self.0.insert(filename, content);
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    println!("Now we have a config {:?}", conf);
    println!("Calling unzip on {}", conf.fp);
    let relevant_files = get_relevant_files(&conf.fp)?;    
    let actions = serialize_actions(relevant_files)?;

    // println!("{:?}\n", actions[0].citations);
    // println!("{:?}\n", actions[0].action);
    // println!("{:?}\n", actions[0].metadata);
    // println!("{:?}\n", actions[0].children);
    // let tree = build_tree(actions);
    
    Ok(())
}

/// Groups related files and parses them into ProvNodes
/// Returns: A vector of ProvNodes, which can be organized into a tree elsewhere
pub fn serialize_actions(relevant_files: RelevantFiles) -> Result<Vec<ProvNode>, serde_yaml::Error> {
    // unpack the HashMap so we can access it here without indirection
    let RelevantFiles(rel_files) = &relevant_files;

    // use filenames to group metadata, citation, and action files
    // Separate terminal and other actions
    let mut leaf_files = Vec::new();
    let mut other_files = Vec::new();
    
    for filename in rel_files.keys() {
        if filename.contains("artifacts"){
            other_files.push(filename.clone());
        } else {
            leaf_files.push(filename.clone());
        }
    }

    // println!("\n\n");
    // for file in &other_files {
    //     println!("{}", file);
    // }
    
    // Capture terminal action
    let leaf = ProvNode::new(leaf_files, relevant_files)?;

    // // Turn a string into an owned PathBuf
    // println!("{}", PathBuf::from(&files.filenames[0]).display());
    // let mut filenames = files.filenames.iter();
    // let filenames = filenames.map(|fp| PathBuf::from(fp));
    // we can zip iterators with itertools.izip
    
    // serialize each and pack them in ProvNodes
    
    let result = vec!(leaf);
    Ok(result)
}


// pub fn build_tree(actions: Vec<Action>) -> ProvNode {
//     // TODO: implement;
//     let result = ProvNode::new();
//     result
// }

/// Opens a .qza or .qzv, harvests relevant files and reads them into memory
/// as strings.
/// 
/// Requires: a valid .qza or .qzv with archive version == 5
/// Returns: RelevantFiles, a vec<NamedFile>
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
                     & (name.contains("action.yaml")
                     |  name.contains("metadata.yaml")
                     |  name.contains("citations.bib")))
        .map(|name| {String::from(name)})
        .collect();

    // Read files into memory, mapping filename to contents
    let mut rel_files = RelevantFiles::new();
    for i in 0..filenames.len() {
        let mut tmp_contents = String::new();
        zip.by_name(&filenames[i]).unwrap().read_to_string(&mut tmp_contents).unwrap();
        rel_files.insert(filenames[i].clone(), tmp_contents);
    }

    Ok( rel_files )
}