/*
MANDY PROCESSORS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "serde_yaml"
/// crate to deserialize YAML.
use serde_yaml;

/// Importing the "Entity"
/// enum from the "coutils"
/// crate to determine the type
/// of file.
use coutils::Entity;

/// Importing the "read_file"
/// method from the "coutils"
/// crate to read text files.
use coutils::read_file;

/// Importing the "FileEntry"
/// struct to work with files easier.
use coutils::FileEntry;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// Importing the "from_str"
/// method from the "serde_json"
/// crate to store JSON
/// into Rust data structures.
use serde_json::from_str;

/// Importing the "clean_split"
/// method from the "coutils"
/// crate to split strings.
use coutils::clean_split;

/// Importing Rust's standard
/// "HashMap" data structure.
use std::collections::HashMap;

/// Importing the method to store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// Deserializing JSON data from the "data"
/// directory and further processing this.
pub fn deserialize_data_json(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, Vec<HashMap<String, String>>>, 
MandyError
> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let map: Vec<HashMap<String, String>> = match from_str(&v) {
                Ok(map) => map,
                Err(e) => {
                    let msg: String = format!("Error in file \"{}.json\":\n{}", file_name, e);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(MandyError::new(&msg.to_string()));
                }
            };
            result.insert(file_name.to_owned(), map);
        }
    }
    Ok(result)
}

/// Deserializing YAML data from the "data"
/// directory and further processing this.
pub fn deserialize_data_yaml(
    data_strings: Vec<HashMap<String, String>>
) -> Result<
HashMap<String, Vec<HashMap<String, String>>>, 
MandyError
> {
    let mut result: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for item in data_strings.into_iter() {
        for (k,v) in item.into_iter() {
            let file_name: &String = &k;
            let map: Vec<HashMap<String, String>> = match serde_yaml::from_str(&v) {
                Ok(map) => map,
                Err(e) => {
                    let msg: String = format!("Error in file \"{}.yaml\":\n{}", file_name, e);
                    return Err::<HashMap<String, Vec<HashMap<String, String>>>, MandyError>(MandyError::new(&msg.to_string()));
                }
            };
            result.insert(file_name.to_owned(), map);
        }
    }
    Ok(result)
}

/// Storing the file contents
/// from data files into
/// a list of maps.
pub fn find_data_files(dir: &String) -> Result<Vec<DataFile>, MandyError> {
    let mut result: Vec<DataFile> = Vec::new();
    let dir_items: Vec<FileEntry> = match list_dir_contents(dir){
        Ok(dir_items) => dir_items,
        Err(e) => {
            return Err::<Vec<DataFile>, MandyError>(MandyError::new(&e.to_string()));
        }
    };
    for item in dir_items {
        if &item.file_type == &Entity::File {
            let file_contents: &String = &match read_file(&item.name){
                Ok(file_contents) => file_contents,
                Err(e) => {
                    return Err::<Vec<DataFile>, MandyError>(MandyError::new(&e.to_string()));
                }
            };
            let path_list: &Vec<String> = 
                &clean_split(
                    &item.name,
                    &String::from("/")
                );
            let data_file: &String = &path_list[path_list.len()-1];
            if data_file.contains(".json"){
                let data_file_name_components: &Vec<String> = &clean_split(
                    &data_file,
                    &String::from(".json")
                );
                let template_key: &String = &data_file_name_components[0];
                let df_inst: DataFile = DataFile::new(file_contents, &DataFileType::JsonData, &template_key);
                result.push(df_inst);
            }
            else if data_file.contains(".yaml"){
                let data_file_name_components: &Vec<String> = &clean_split(
                    &data_file,
                    &String::from(".yaml")
                );
                let template_key: &String = &data_file_name_components[0];
                let df_inst: DataFile = DataFile::new(file_contents, &DataFileType::YamlData, &template_key);
                result.push(df_inst);
            }
            else {
                return Err::<Vec<DataFile>, MandyError>(
                    MandyError::new(
                        &format!("Unknown file type: {}\n", &item.name)
                    )
                );
            }
        }
        else {}
    }
    return Ok(result);
}

/// An enum that
/// describes all
/// different
/// data formats that
/// Mandy accepts.
#[derive(Debug, Clone)]
pub enum DataFileType {
    JsonData,
    YamlData,
    NoData
}

/// A data structure to
/// hold information about
/// a found data file.
#[derive(Debug, Clone)]
pub struct DataFile {
    pub contents: String,
    pub file_type: DataFileType,
    pub file_prefix: String
}

/// Implementing generic
/// methods for the "DataFile"
/// data structure.
impl DataFile {

    /// A generic method
    /// to create a new instance
    /// of this data structure.
    pub fn new(
        contents: &String,
        file_type: &DataFileType,
        file_prefix: &String
    ) -> DataFile {
        DataFile {
            contents: contents.to_owned(), 
            file_type: file_type.to_owned(), 
            file_prefix: file_prefix.to_owned()
        }
    }
}

/// This method accepts a list
/// of "DataFile" data structures
/// and creates a vector of
/// "HashMap" data structures.
pub fn hashmaps_from_files(
    subject: &Vec<DataFile>
) -> Vec<HashMap<String, String>> {
    let mut res_vec: Vec<HashMap<String, String>> = Vec::new();
    for item in subject {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(
            item.file_prefix.clone(), 
            item.contents.clone()
        );
        res_vec.push(map);
    }
    res_vec
}

/// A data structure to
/// hold information about
/// a found data directory.
#[derive(Debug, Clone)]
pub struct DataDir {
    pub files: Vec<HashMap<String, String>>,
    pub file_type: DataFileType
}

/// Implementing generic
/// methods for the "DataDir"
/// data structure.
impl DataDir{

    /// A generic method
    /// to create a new instance
    /// of this data structure.
    pub fn new(
        files: &Vec<HashMap<String, String>>, 
        file_type: &DataFileType
    ) -> DataDir {
        DataDir {
            files: files.to_owned(),
            file_type: file_type.to_owned()
        }
    }
}

/// This method converts a series of "DataFile" data structures
/// into a single "DataDir" data structure instance.
pub fn data_dir_from_files(subject: &Vec<DataFile>) -> DataDir {
    let data_file_type: DataFileType = subject[0].clone().file_type;
    return DataDir::new(
        &hashmaps_from_files(subject),
        &data_file_type
    );
}