use std::fs::File;
use std::io::Read;
use std::env::current_exe;
use std::fs::DirEntry;
use std::path::Path;

/*
 * Represents a document read from a file
 */
#[derive(Serialize, Deserialize)]
pub struct Document {
    /*
     * Title of the document
     */
    title: String,

    /*
     * Body of the Document
     */
    body: String,

    /*
     * URL of Document
     */
    url: String,
}

/*
 * Contains operations of the Document
 */
impl Document {
    /*
     * Retrieves the title of the Document
     * Cloned to preserve data integrity
     */
    pub fn getTitle(&self) -> String {
        self.title.clone()
    }

    /*
     * Retrieves the body of the Document.
     * Cloned to preserve data integrity
     */
    pub fn getBody(&self) -> String {
        self.body.clone()
    }

    /*
     * Retrieves the URL of the Document
     * cloned to preserve data integrity
     */
    pub fn getURL(&self) -> String {
        self.url.clone()
    }

    /*
     * Clones the Document and returns the Clone
     */
    pub fn clone(&self) -> Document {
        Document {
            title: self.title.clone(),
            body: self.body.clone(),
            url: self.url.clone(),
        }
    }
}

/*
 * Returns the string representation of a Path/DirEntry
 */
pub fn path_to_string(file: DirEntry) -> String {
    let mut filePath = file.path();
    filePath.to_str().expect("No String").to_string()
}

/*
 * Reads a File given a file name and returns the Document representation of the File
 */
pub fn read_file(file_name: &str) -> Document {
    let filePath = Path::new(file_name);
    let extension = filePath.extension().expect("Not a valid extension");

    if extension == "json" {
        return read_json_file(file_name);
    }
    return read_text_file(file_name);
}

/*
 * Reads a regular text file and returns Document representation
 */
pub fn read_text_file(file_name: &str) -> Document {
    let mut file = File::open(file_name).expect("File not Found");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Failed to retrieve contents",
    );
    /*
     * While the content of the file document is copied to the body,
     * the title is set to the file name and the url is set to blank
     */
    Document {
        title: file_name.to_string(),
        body: contents,
        url: "".to_string(),
    }

}

pub fn read_json_file(file_name: &str) -> Document {
    let mut file = File::open(file_name).expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Error reading contents",
    );
    let mut document: Document =
        ::serde_json::from_str(&contents).expect("Error retrieving JSON document");
    let mut document_clone = document.clone();
    let mut collection = document_clone.body.split_whitespace();
    let mut newBody: String = "".to_owned();
    for token in collection {
        let newString = format!("{} ", token);
        newBody.push_str(newString.as_str());
    }
    document.body = newBody;
    return document;
}
