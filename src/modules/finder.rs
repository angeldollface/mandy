/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "Entity"
/// enum from the "coutils"
/// crate to determine the type
/// of file.
use coutils::Entity;

/// Importing the "FileEntry"
/// struct to work with files easier.
use coutils::FileEntry;

/// Importing the method store
/// information about a directory's
/// contents.
use coutils::list_dir_contents;

/// A data structure
/// to store info about
/// Markdown files.
pub struct MDFile {
    pub dir: String,
    pub file: String
}

/// Implementing standard structures for
/// the "MDFile" structure.
impl MDFile {

    /// A method to create a new instance of this structure.
    pub fn new(dir: &String, file: &String) -> MDFile {
        return MDFile { dir: dir.to_owned(), file: file.to_owned() }
    }

    /// A method to create a string representation of this structure.
    pub fn to_string(&self) -> String {
        return format!("{} => {}", &self.dir, &self.file);
    }
}

/// Finds all ".markdown" files in a given directory.
pub fn find_md_files(project_dir: &String) -> Vec<MDFile> {
    let mut result: Vec<MDFile> = Vec::new();
    let entries: Vec<FileEntry> = list_dir_contents(project_dir);
    for entry in entries {
        if entry.name.contains(&".markdown") {
            result.push(
                MDFile::new(&project_dir,&entry.name)
            );
        }
        else if entry.file_type == Entity::Dir {
            let dir_entries: Vec<FileEntry> = list_dir_contents(&entry.name);
            for sl_entry in dir_entries {
                if sl_entry.name.contains(&".markdown") 
                   && sl_entry.file_type == Entity::File {
                    let parent_dir: &String = &entry.name;
                    let md_file: MDFile = MDFile::new(parent_dir, &sl_entry.name);
                    result.push(md_file);
                }
                else {}
            }
        }
    }
    return result;
}