use std::fs::File;
use std::io::Read;
use std::env::current_exe;
use std::fs::DirEntry;
use std::path::Path;

pub fn path_to_string(file:DirEntry) -> String {
    let mut filePath = file.path();
    filePath.to_str().expect("No String").to_string()
}

pub fn read_file(file_name: &str) -> String {
    let filePath = Path::new(file_name);
    let extension = filePath.extension().expect("Not a valid extension");

    if extension == ".json" {
        return read_json_file(file_name); 
    }
    return read_text_file(file_name);
}

pub fn read_text_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not Found");
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to retrieve contents");
    return contents;
}

pub fn read_json_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(& mut contents).expect("Error reading contents");
    ::serde_json::from_str(&contents).expect("Error retrieving JSON document")
}
