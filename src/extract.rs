use regex::Regex;
use std::{fs, path::Path};

struct Extractor {
    contents: Vec<String>,
    matches: Vec<String>,
    filename: String,
}

pub fn read_contents(filename: &Path) -> String {
    fs::read_to_string(filename).expect("Error reading file!")
}

impl Extractor {
    fn new() -> Self {
        Self {
            contents: Vec::new(),
            matches: Vec::new(),
            filename: String::new(),
        }
    }

    fn email(&self) -> Vec<String> {
        fs::read_to_string(self.filename)
            .expect("Error reading file!")
            .lines()
            .filter(|l| {
                let re = Regex::new(r#"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$"#).unwrap();
                re.is_match(l)
            })
            .collect()
    }
}

pub fn extractor(filename: &str) {
    let x = Extractor::new();
    read_contents(Path::new(filename));
}
