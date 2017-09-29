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

pub fn addToPath(pathbuf:&mut PathBuf, add: &str) -> bool {
    let mut testPath = pathbuf.clone();
    testPath.push(add);

    if verifyPath(testPath) {
        pathbuf.push(add);
        return true;
    }
    return false;
}

pub fn changeDirectory(pathbuf:&mut PathBuf, new: &str) -> bool {
    pathbuf.pop();
    addToPath(pathbuf, new)
}

pub fn verifyPath(pathbuf: PathBuf) -> bool {
    if pathbuf.exists() {
        return true; 
    }
    println!("{} does not exist", pathbuf.display()); 
    return false;
}
