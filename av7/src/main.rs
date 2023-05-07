use std::fs;

struct Command {
    name: String,
    args: Vec<String>,
    output: Vec<String>,
}

struct FileSystemEntry {
    name: String,
    size: i32,
    content: Option<Vec<FileSystemEntry>>,
}

fn main() {
    let file_path = "./data.txt";
    let file_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
}
