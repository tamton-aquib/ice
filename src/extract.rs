use regex::Regex;
use std::{fs, path::Path};

struct Extractor {
    contents: String,
}

impl Extractor {
    fn new(filename: &str) -> Self {
        let filepath = Path::new(filename);
        Self {
            contents: fs::read_to_string(filepath).unwrap(),
        }
    }

    fn emails(&self) -> Vec<String> {
        Regex::new(r"[a-z0-9_+][a-z0-9_+.]*[a-z0-9_+]?@[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}")
            .unwrap()
            .captures_iter(&self.contents)
            .map(|i| i[0].to_string())
            .collect()
    }

    fn phones(&self) -> Vec<String> {
        Regex::new(r"[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{4})")
            .unwrap()
            .captures_iter(&self.contents)
            .map(|i| i[0].to_string())
            .collect()
    }

    fn ips(&self) -> Vec<String> {
        Regex::new(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}")
            .unwrap()
            .captures_iter(&self.contents)
            .map(|i| i[0].to_string())
            .collect()
    }
}

pub fn extractor(xtype: &str, filename: &str) -> String {
    let extract = Extractor::new(filename);
    let matches = match xtype {
        "email" => extract.emails(),
        "phone" => extract.phones(),
        "ip" => extract.ips(),
        _ => todo!(),
    };

    matches.join("\n")
}
