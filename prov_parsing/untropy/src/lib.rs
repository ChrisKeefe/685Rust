// TODO: remove
// #![allow(warnings)]

use std::error::Error;
use std::fs::File;
use std::io::Read;
// use std::path::PathBuf;
// use itertools::izip;
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
    // No need to capture Execution or Environment details for now
    // serde gracefully drops missing keys by default.
    // execution: Execution,
    action: ActionDetails,
    // environment: serde_yaml::Value
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
    // This could probably be an Option(String), but we'll capture nulls as 
    // strings for now
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
    // pub fn new(metadata: ActionMetadata, action: Action) -> ProvNode {
    //     ProvNode {
    //         metadata,
    //         action,
    //         citations: None,
    //         children: None,
    //     }
    // }

    pub fn new(filenames: Vec<String>) -> Result<ProvNode, serde_yaml::Error> {
        let mut metadata: Option<ActionMetadata> = None;
        let mut action: Option<Action> = None;
        let mut citations = None;
        for i in filenames {
            if i.contains("metadata.yaml") {
                println!("in metadata");
                metadata = serde_yaml::from_str(&i)?;
            } else if i.contains("action.yaml") {
                println!("in action");
                action = serde_yaml::from_str(&i)?;
            } else if i.contains("citations.bib") {
                println!("in citations");
                citations = Some(i);
            }
        }

        Ok(ProvNode { metadata, action, citations, children: None })
    }
}

// TODO: Would this work better as a HashMap? Or maybe we drop this type, and
// just build a Vec<NamedFile>
#[derive(Debug)]
pub struct RelevantFiles ( Vec<NamedFile> );

impl RelevantFiles {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub struct NamedFile {
    pub filename: String,
    pub content: String,
}

impl NamedFile {
    pub fn new(filename: String, content: String) -> NamedFile {
        NamedFile {filename, content}
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    println!("Now we have a config {:?}", conf);
    println!("Calling unzip on {}", conf.fp);
    let relevant_files = get_relevant_files(&conf.fp)?;
    // TODO: remove diagnostics
    // println!("\nFirst archive contains: ");
    // println!("{}", relevant_files.contents[0]);
    
    let actions = serialize_actions(relevant_files)?;
    // let tree = build_tree(actions);
    
    Ok(())
}

/// Groups related files and parses them into ProvNodes
/// 
/// Returns: A vector of ProvNodes, which can be organized into a tree elsewhere
pub fn serialize_actions(rel_files: RelevantFiles) -> Result<ProvNode, serde_yaml::Error> {
    // use filenames to group metadata, citation, and action files
    
    // Separate terminal and other actions
    let mut leaf_files = Vec::new();
    let mut other_files = Vec::new();
    
    for i in 0..rel_files.len() {
        let tmp_filename = rel_files.0[i].filename.clone();
        // println!("{}", tmp_filename);
        if tmp_filename.contains("artifacts"){
            other_files.push(tmp_filename);
        } else {
            leaf_files.push(tmp_filename);
        }
    }

    // println!("\n\n");
    // for file in leaf_files {
    //     println!("{}", file);
    // }
    
    // println!("\n\n");
    // for file in other_files {
    //     println!("{}", file);
    // }
    
    // Capture terminal action
    // let mut leaf = ProvNode::New();
    // Capture terminal action
    // let mut leaf = ProvNode::New();

    // // Turn a string into an owned PathBuf
    // println!("{}", PathBuf::from(&files.filenames[0]).display());
    // let mut filenames = files.filenames.iter();
    // let filenames = filenames.map(|fp| PathBuf::from(fp));
    // we can zip iterators with itertools.izip

    
    // serialize each and pack them in ProvNodes
    
    let result = ProvNode::new(leaf_files)?;
    // println!("{:?}", result);
    
    
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
    let mut files = Vec::new();
    for i in 0..filenames.len() {
        let mut tmp_contents = String::new();
        zip.by_name(&filenames[i]).unwrap().read_to_string(&mut tmp_contents).unwrap();
        files.push(NamedFile::new(filenames[i].clone(), tmp_contents));
    }

    Ok(RelevantFiles( files ))
}