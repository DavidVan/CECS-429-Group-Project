use std::env::current_exe;
use std::path::PathBuf;

/*
 * Constructs a project path that will begin at the assets folder of the project
 *
 * # Returns
 *
 * The initialized PathBuf set at CECS-429-Group-Project/search_engine/assets
 */
pub fn initialize_path() -> PathBuf {
    let mut index_path = current_exe().expect("Not a valid path");

    while !index_path.ends_with("CECS-429-Group-Project") {
        index_path.pop();
    }
    index_path.push("search_engine");
    index_path.push("assets");
    return index_path;
}

/*
 * Attempts to append a directory to the path buffer.
 *
 *
 * # Arguments
 *
 * *`path_buf` - The Path Buffer currently set at the directory
 * *`add` - The directory that will be added to the path
 *
 * # Returns
 * 
 * True if the add was successful, false otherwise
 */
fn add_to_path(path_buf: &mut PathBuf, add: &str) -> bool {
    let mut test_path = path_buf.clone();
    test_path.push(add);

    if verify_path(test_path) {
        path_buf.push(add);
        return true;
    }
    return false;
}

/*
 * Attempts to change the directory of the path buffer to specified directory
 *
 * # Arguments
 *
 * *`path_buf` - The Path Buffer currently set at the directory
 * *`new` - The new directory that the Path Buffer will be set to
 *
 * # Returns
 *
 * True if the change was successful, false otherwise
 */
pub fn change_directory(path_buf: &mut PathBuf, new: &str) -> bool {
    let pathbuf_clone = path_buf.clone();
    let current = pathbuf_clone.file_name().expect("Not a valid os string");
    let current_str = current.to_str().expect("Not a valid string");
    if new == current {
        return false;
    }
    if current == "assets" {
        return add_to_path(path_buf, new);
    }
    path_buf.pop();
    let success: bool = add_to_path(path_buf, new);
    if !success {
        add_to_path(path_buf, current_str);
    }
    return success;
}

/*
 * Verifies if a path buffer exists and is a directory
 *
 * # Arguments
 *
 * *`path_buf` - The Path Buffer set at a directory and will be tested if valid
 *
 * # Returns
 *
 * True if the Path Buffer is set at a valid and existing directory
 * False otherwise
 */
pub fn verify_path(path_buf: PathBuf) -> bool {
    if path_buf.exists() && path_buf.is_dir() {
        return true;
    }
    println!("{} does not exist", path_buf.display());
    return false;
}
