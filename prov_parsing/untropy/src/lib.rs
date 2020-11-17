// TODO: remove
// #![allow(warnings)]

// TODO: Can we drop std::error::Error in favor of std::io::Error, and lose the
// `as ioError`?
use std::error::Error;
use std::io::Error as ioError;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

/// Contents of a QIIME 2 Archive, including Archive UUID and a HashMap of
/// filename: content pairs
#[derive(Debug)]
pub struct ArchiveContents { root_uuid: String,
                             file_contents: HashMap<String, String> }

impl ArchiveContents {
    pub fn len(&self) -> usize {
        self.file_contents.len()
    }

    pub fn new(root_uuid: &str) -> ArchiveContents {
        let root_uuid = String::from(root_uuid);
        let file_contents = HashMap::new();
        ArchiveContents { root_uuid, file_contents }
    }

    pub fn insert(&mut self, filename: String, content: String) {
        self.file_contents.insert(filename, content);
    }
}

/// One node of a provenance tree
#[derive(Debug, Deserialize, Serialize)]
pub struct ProvNode {
    uuid: Option<String>,
    metadata: Option<ActionMetadata>,
    action: Option<Action>,
    citations: Option<String>,
    children: Option<Vec<ProvNode>>,
}

impl ProvNode {
    pub fn new(filenames: Vec<String>, rel_files: &ArchiveContents) 
            -> Result<ProvNode, serde_yaml::Error> {
        let mut metadata: Option<ActionMetadata> = None;
        let mut action: Option<Action> = None;
        let mut citations = None;
        let key_err = "Key Error in ProvNode::new(); Filepath does not exist in ArchiveContents";
        for i in filenames {
            let content = rel_files.file_contents.get(&i).ok_or_else(|| {key_err});
            if i.contains("metadata.yaml") {
                metadata = serde_yaml::from_str(content.unwrap())?;
            } else if i.contains("action.yaml") {
                action = serde_yaml::from_str(content.unwrap())?;
            } else if i.contains("citations.bib") {
                citations = Some(String::from(content.unwrap()));
            }
        }

        // TODO: Handle errors more explicitly
        let uuid = Some(metadata.as_ref().unwrap().uuid.clone());

        Ok(ProvNode { uuid, metadata, action, citations, children: None })
    }
}

/// Main run function for the program - primary program logic lives here
pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let relevant_files = get_relevant_files(&conf.fp)?;    
    let actions = serialize_actions(relevant_files)?;

    // println!("{:?}", actions[0].uuid);
    // println!("{:?}\n", actions[0].citations);
    // println!("{:?}\n", actions[0].action);
    // println!("{:?}\n", actions[0].metadata);
    // println!("{:?}\n", actions[0].children);
    // println!("");
    // let tree = build_tree(actions);
    
    Ok(())
}

/// Groups related files by Action UUID and parses them into ProvNodes
/// Returns: A vector of ProvNodes, which can be organized into a tree elsewhere
pub fn serialize_actions(relevant_files: ArchiveContents) -> Result<Vec<ProvNode>,
Box<dyn Error>> {
    // TODO: Check the QIIME2 archive version, and handle appropriately.
    // For now, that probably means error if version != 5

    // use filenames to group metadata, citation, and action files into
    // terminal action (our archive root/analysis leaf) and other actions
    let mut leaf_filenames = Vec::new();
    let mut other_filenames = Vec::new();
    
    for filename in relevant_files.file_contents.keys() {
        if filename.contains("artifacts"){
            other_filenames.push(filename.clone());
        } else {
            leaf_filenames.push(filename.clone());
        }
    }
    
    // Filepaths for terminal actions are ready to be read into a node
    let mut prev_id:PathBuf  = PathBuf::from(&relevant_files.root_uuid);
    let mut files_for_one_action: Vec<PathBuf> = leaf_filenames.iter()
        .map(|fp| PathBuf::from(fp))
        .collect();

        // TODO: remove
        // println!("\n\n");
        // for file in &other_filenames {
            //     println!("{}", file);
            // }
            
    // We'll group files by action, by sorting their paths. We'll building a
    // node for the previous group whenever the UUID changes
    other_filenames.sort();

    let path_prefix = PathBuf::from(&relevant_files.root_uuid)
        .join("provenance")
        .join("artifacts");
    
    let mut result: Vec<ProvNode> = Vec::new();

    for filename in &other_filenames {
        println!();
        let this_path = PathBuf::from(filename);
        let suffix = this_path.strip_prefix(&path_prefix)?;
        let path_components: Vec<_> = (&suffix).components()
            .map(|comp| comp.as_os_str())
            .collect();        
        let action_uuid = &PathBuf::from(path_components[0]);
        
        // Create a new ProvNode for each UUID
        // TODO: remove
        // println!("Current id: {:?}", action_uuid);
        // println!("Prev id: {:?}", &prev_id);
        if action_uuid != &prev_id{
            // Make a ProvNode out of the previous bunch of files...
            // TODO: remove
            println!("files for one action: {:?}", files_for_one_action);
            let node = ProvNode::new(files_for_one_action.iter()
                .map(|file| String::from(file.to_str().unwrap()))
                .collect()
                , &relevant_files)?;
            result.push(node);
            // TODO: remove
            // println!("\n\nNodes: {:?}\n", result.len());
                                     
            // ...then create a new vector to hold the next bunch
            prev_id = PathBuf::from(action_uuid);
            files_for_one_action = Vec::new();
        }
        files_for_one_action.push(PathBuf::from(filename));
        // TODO: remove
        // println!("Files for one action after reset: {:?}", files_for_one_action);
    }
    
    // Create one final ProvNode from the last bunch of files
    let node = ProvNode::new(files_for_one_action.iter()
        .map(|file| String::from(file.to_str().unwrap()))
        .collect()
        , &relevant_files)?;

    result.push(node);

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
/// Requires: user pass path to a valid .qza or .qzv with archive version == 5
pub fn get_relevant_files(fp: &str) -> Result<ArchiveContents, Box<dyn Error>> {
    // Get a filepath and create a ZipArchive
    let fp = File::open(fp)?;
    let mut zip = zip::ZipArchive::new(fp)?;
    
    // TODO: filtering these as Paths would allow us to consider the semantics
    // of whole components, rather than using the `/data` hack to exclude data
    // directory items but keep `metadata`
    let top_level_metadata: Vec<String> = zip.file_names()
    .filter(|name| !name.contains("provenance"))
    .filter(|name| !name.contains("/data"))
    .filter(|name| name.contains("metadata.yaml"))
    .map(|name| {String::from(name)})
    .collect();
    
    // for name in &top_level_metadata{
    //     println!("{}", name);
    // };
    
    let mut rel_files; 
    let n_files_captured = top_level_metadata.len();
    if n_files_captured == 1 {
        let filename = top_level_metadata[0].clone();
        let reader = zip.by_name(&filename)?;
        let tmp_md: ActionMetadata = serde_yaml::from_reader(reader)?;
        rel_files = ArchiveContents::new( &tmp_md.uuid );
    } else {
        return Err(Box::new(ioError::new(std::io::ErrorKind::InvalidInput,
                            "Malformed Archive: Multiple top-level metadata.yaml files")));
    }
    
    // Create a positive mask for relevant files
    let filenames: Vec<String> = zip.file_names()
        .filter(|name| name.contains("provenance") 
                     & (name.contains("action.yaml")
                     |  name.contains("metadata.yaml")
                     |  name.contains("citations.bib")))
        .map(|name| {String::from(name)})
        .collect();

    // Read files into memory, mapping filename to contents
    for i in 0..filenames.len() {
        let mut tmp_contents = String::new();
        zip.by_name(&filenames[i]).unwrap().read_to_string(&mut tmp_contents).unwrap();
        rel_files.insert(filenames[i].clone(), tmp_contents);
    }

    Ok( rel_files )
}