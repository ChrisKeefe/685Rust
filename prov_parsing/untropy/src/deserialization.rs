use serde::{Deserialize, Serialize};
use std::io::Error as ioError;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::path::{PathBuf};

// Type aliases for now. Convert to NewType pattern when/if we tackle validation
type UUID = String;
type SemanticType = String;

/// Select contents of an action.yaml file
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
    pub action: ActionDetails,
    // No need to capture the details in Execution or Environment objects for now
    // serde gracefully drops missing keys by default.
}

/// Data from the action tag in an action.yaml
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionDetails {
    #[serde(rename="type")]
    pub semantic_type: String,
    pub plugin: Option<String>,
    pub action: Option<String>,
    // TODO: Make this a tuple?
    pub inputs: Option<Vec<HashMap<SemanticType, UUID>>>,
    pub parameters: Option<serde_yaml::Value>,
    #[serde(rename="output-name")]
    pub output_name: Option<String>,
    // TODO: what even is alias-of?
    // #[serde(rename="alias-of")]
    // alias_of: String,
    // params: HashMap<String, String>,
}

/// Contents of a metadata.yaml file
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionMetadata {
    pub uuid: String,
    #[serde(rename="type")]
    pub semantic_type: String,
    // We'll capture nulls as Strings instead of Option(String)s for simplicity
    pub format: String,
}

/// Contents of a QIIME 2 Archive, including Archive UUID and a HashMap of
/// filename: content pairs
#[derive(Debug)]
pub struct ArchiveContents { pub root_uuid: String,
                             pub file_contents: HashMap<String, String> }

impl ArchiveContents {
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
/// NOTE: citations temporarily removed for readability/presentation purposes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProvNode {
    pub uuid: Option<String>,
    pub metadata: Option<ActionMetadata>,
    pub action: Option<Action>,
    // pub citations: Option<String>,
    pub parents: Option<Vec<ProvNode>>,
}

impl ProvNode {
    pub fn new(filenames: Vec<String>, rel_files: &ArchiveContents) 
            -> Result<ProvNode, serde_yaml::Error> {
        let mut metadata: Option<ActionMetadata> = None;
        let mut action: Option<Action> = None;
        // let mut citations = None;
        let key_err = "Key Error in ProvNode::new(); Filepath does not exist in ArchiveContents";
        for i in filenames {
            let content = rel_files.file_contents.get(&i).ok_or_else(|| {key_err});
            if i.contains("metadata.yaml") {
                metadata = serde_yaml::from_str(content.unwrap())?;
            } else if i.contains("action.yaml") {
                action = serde_yaml::from_str(content.unwrap())?;
            } 
            // else if i.contains("citations.bib") {
            //     citations = Some(String::from(content.unwrap()));
            // }
        }   

        let uuid = Some(metadata.as_ref().unwrap().uuid.clone());

        // Ok(ProvNode { uuid, metadata, action, citations, parents: None })
        Ok(ProvNode { uuid, metadata, action, parents: None })
    }
}

/// Takes in an unordered list of Nodes, and links them through "parent" refs
/// Returns the root node of the Provenance Tree
pub fn build_tree(actions: &mut Vec<ProvNode>) -> Result<&ProvNode, Box<dyn Error>> {
    // Get Input UUIDs from root node, and add them as parent
    for i in 0..actions.len() {
        // Get "parent artifact" Hashmaps
        if let Some(parents) = &actions[i].action.as_ref().unwrap().action.inputs {
            // This action has Some parents - get their UUIDs
            let uuids: Vec<UUID> = parents.iter()
                .map(|parent| parent.values().next().unwrap().to_string().clone())
                .collect::<Vec<UUID>>();
            
            // Look up these UUIDs and add the matching ProvNode to tree
            // TODO: Remove
            // println!("{:?}", uuids);
            let relevant_nodes: Vec<ProvNode> = actions.iter().
                filter(|action| uuids.contains(&action.uuid.as_ref().unwrap()))
                .map(|action| action.to_owned())
                .collect();

            actions[i].parents = Some(relevant_nodes);            

        }
    }

    // Return the root node of the tree.
    Ok(&actions[0])
}

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

    // We'll group files by action, by sorting their paths. We'll building a
    // node for the previous group whenever the UUID changes
    other_filenames.sort();

    let path_prefix = PathBuf::from(&relevant_files.root_uuid)
        .join("provenance")
        .join("artifacts");
    
    let mut result: Vec<ProvNode> = Vec::new();

    for filename in &other_filenames {
        let this_path = PathBuf::from(filename);
        let suffix = this_path.strip_prefix(&path_prefix)?;
        let path_components: Vec<_> = (&suffix).components()
            .map(|comp| comp.as_os_str())
            .collect();        
        let action_uuid = &PathBuf::from(path_components[0]);
        
        // Create a new ProvNode for each UUID
        if action_uuid != &prev_id{
            // Make a ProvNode out of the previous bunch of files...
            let node = ProvNode::new(files_for_one_action.iter()
                .map(|file| String::from(file.to_str().unwrap()))
                .collect()
                , &relevant_files)?;
            result.push(node);
                                     
            // ...then create a new vector to hold the next bunch
            prev_id = PathBuf::from(action_uuid);
            files_for_one_action = Vec::new();
        }
        files_for_one_action.push(PathBuf::from(filename));
    }
    
    // Create one final ProvNode from the last bunch of files
    let node = ProvNode::new(files_for_one_action.iter()
        .map(|file| String::from(file.to_str().unwrap()))
        .collect()
        , &relevant_files)?;

    result.push(node);

    Ok(result)
}

