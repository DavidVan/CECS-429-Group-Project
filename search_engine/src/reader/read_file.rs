use std::fs::File;
use std::io::Read;
use std::env::current_exe;
use std::fs::DirEntry;
use std::path::Path;

pub fn path_to_string(file:DirEntry) -> String {
    let mut filePath = file.path();
    filePath.to_str().expect("No String").to_string()
}

pub fn read_text_file(fileName: &str) -> String {
    let mut file = File::open(fileName).expect("File not Found");
    
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to retrieve content");
    return content;
}
