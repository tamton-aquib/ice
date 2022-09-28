use std::{fs, path::Path};

// NOTE: Might be harder than it seems
pub fn read_binary(s: &str, query: &str) -> String {
    let contents = fs::read_to_string(Path::new(s)).expect("File reading error!");
    let refined = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&str>>();
    println!("{refined:?}");
    "noice".to_string()
}
