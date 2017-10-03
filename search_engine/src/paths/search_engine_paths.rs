use std::env::current_exe;
use std::path::PathBuf;

pub fn initializePath() -> PathBuf {
    let mut documentPath = current_exe().expect("Not a valid path");

    while !documentPath.ends_with("CECS-429-Group-Project") {
        documentPath.pop();
    }
    documentPath.push("search_engine");
    documentPath.push("assets");
    println!("{}", documentPath.display());
    return documentPath;
}

fn addToPath(pathbuf: &mut PathBuf, add: &str) -> bool {
    let mut testPath = pathbuf.clone();
    testPath.push(add);

    if verifyPath(testPath) {
        pathbuf.push(add);
        return true;
    }
    return false;
}

pub fn changeDirectory(pathbuf: &mut PathBuf, new: &str) -> bool {
    let pathbuf_clone = pathbuf.clone();
    let mut current = pathbuf_clone.file_name().expect("Not a valid os string");
    let mut current_str = current.to_str().expect("Not a valid string");
    if new == current {
        return false;
    }
    if current == "assets" {
        return addToPath(pathbuf, new);
    }
    pathbuf.pop();
    let success: bool = addToPath(pathbuf, new);
    if !success {
        addToPath(pathbuf, current_str);
    }
    return success;
}

pub fn verifyPath(pathbuf: PathBuf) -> bool {
    if pathbuf.exists() {
        return true;
    }
    println!("{} does not exist", pathbuf.display());
    return false;
}
