use std::fs;

pub fn read_lines(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    
    contents.lines().map(|x| x.trim_end().to_string()).collect()
}