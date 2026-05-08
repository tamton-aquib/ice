use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::Path;

fn get_input(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        std::fs::read_to_string(input).context("Failed to read file")
    } else {
        Ok(input.to_string())
    }
}

fn emails(content: &str) -> Vec<String> {
    Regex::new(r"[a-z0-9_+][a-z0-9_+.]*[a-z0-9_+]?@[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}")
        .unwrap()
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn phones(content: &str) -> Vec<String> {
    Regex::new(r"[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{4})")
        .unwrap()
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn ips(content: &str) -> Vec<String> {
    Regex::new(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}")
        .unwrap()
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

pub fn extractor(xtype: &str, input: &str) -> Result<String> {
    let content = get_input(input)?;
    let matches = match xtype {
        "email" => emails(&content),
        "phone" => phones(&content),
        "ip" => ips(&content),
        _ => return Err(anyhow!("Unknown extract type: {}", xtype)),
    };

    Ok(matches.join("\n"))
}
