use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

fn get_input(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        std::fs::read_to_string(input).context("Failed to read file")
    } else {
        Ok(input.to_string())
    }
}

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[a-z0-9_+][a-z0-9_+.]*[a-z0-9_+]?@[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}")
        .unwrap()
});

static PHONE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{3})[\.\-)( ]*([0-9]{4})").unwrap()
});

static IP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}").unwrap());

static MAC_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)([0-9a-f]{2}[:-]){5}[0-9a-f]{2}").unwrap());

static FLAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b[a-z0-9_]+\{[^}\n]{2,200}\}").unwrap());

fn emails(content: &str) -> Vec<String> {
    EMAIL_RE
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn phones(content: &str) -> Vec<String> {
    PHONE_RE
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn ips(content: &str) -> Vec<String> {
    IP_RE
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn macs(content: &str) -> Vec<String> {
    MAC_RE
        .captures_iter(content)
        .map(|i| i[0].to_string())
        .collect()
}

fn flags(content: &str) -> Vec<String> {
    FLAG_RE
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
        "mac" => macs(&content),
        "flag" => flags(&content),
        _ => return Err(anyhow!("Unknown extract type: {}", xtype)),
    };

    Ok(matches.join("\n"))
}
