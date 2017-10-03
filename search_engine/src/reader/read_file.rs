use std::fs::File;
use std::io::Read;
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
     *
     * # Returns
     *
     * Clone of the tile attribute of a Document
     */
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    /*
     * Retrieves the body of the Document.
     * 
     * # Returns
     *
     * Clone of the tile attribute of a Document
     */
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    /*
     * Retrieves the URL of the Document
     *
     * # Returns
     *
     * Clone of the tile attribute of a Document
     */
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /*
     * Returns a clone of the Document
     *
     * # Returns
     *
     * A clone representation of the Document
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
 *
 * # Arguments
 *
 * *`file` - The file_path that will translated to a String datatype
 *
 * # Returns
 *
 * The String representatoin of the file
 */
pub fn path_to_string(file: DirEntry) -> String {
    let file_path = file.path();
    file_path.to_str().expect("No String").to_string()
}

/*
 * Reads a File given a file name and returns the Document representation of the File
 *
 * # Arguments
 *
 * *`file_name` - The file that will be read
 *
 * # Returns
 *
 * The read file in Document form
 */
pub fn read_file(file_name: &str) -> Document {
    let file_path = Path::new(file_name);
    let extension = file_path.extension().expect("Not a valid extension");

    if extension == "json" {
        return read_json_file(file_name);
    }
    return read_text_file(file_name);
}

/*
 * Reads a regular text file and returns Document representation
 *
 * # Arguments
 *
 * *`file_name` - The text file that will be read
 *
 * # Returns
 *
 * The processed file in Document form
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

/*
 * Reads a .JSON file and returns the Document representatoin
 *
 * # Arguments
 *
 * *`file_name` - The JSON file that will be read
 *
 * # Returns
 *
 * The Document representation of the JSON file read
 */
pub fn read_json_file(file_name: &str) -> Document {
    let mut file = File::open(file_name).expect("File not Found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(
        "Error reading contents",
    );
    let mut document: Document =
        ::serde_json::from_str(&contents).expect("Error retrieving JSON document");
    let document_clone = document.clone();
    let collection = document_clone.body.split_whitespace();
    let mut new_body: String = "".to_owned();
    for token in collection {
        let new_string = format!("{} ", token);
        new_body.push_str(new_string.as_str());
    }
    document.body = new_body;
    return document;
}
