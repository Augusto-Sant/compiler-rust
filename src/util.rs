pub fn read_file(path: &str) -> String {
    let file_contents = std::fs::read_to_string(path).expect("Error reading file");
    file_contents
}
