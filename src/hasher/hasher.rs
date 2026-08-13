use anyhow::{Context, Result};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn hash_file<D: Digest>(path: &str) -> Result<String> {
    let mut file = File::open(path).context("Failed to open file")?;
    let mut hasher = D::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer).context("Failed to read file")?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect())
}

fn hash_str<D: Digest>(s: &str) -> String {
    let hash = D::digest(s.as_bytes());
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn md5(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        hash_file::<Md5>(input)
    } else {
        Ok(hash_str::<Md5>(input))
    }
}

pub fn sha1(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        hash_file::<Sha1>(input)
    } else {
        Ok(hash_str::<Sha1>(input))
    }
}

pub fn sha256(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        hash_file::<Sha256>(input)
    } else {
        Ok(hash_str::<Sha256>(input))
    }
}

pub fn sha512(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        hash_file::<Sha512>(input)
    } else {
        Ok(hash_str::<Sha512>(input))
    }
}

pub fn hashid(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() {
        return String::from("(empty input)");
    }
    let len = s.len();
    let hex = s.chars().all(|c| c.is_ascii_hexdigit());
    let mut matches: Vec<&str> = Vec::new();
    match (len, hex) {
        (32, true) => matches.push("MD5"),
        (40, true) => matches.push("SHA-1"),
        (56, true) => matches.push("SHA-224"),
        (64, true) => matches.push("SHA-256"),
        (96, true) => matches.push("SHA-384"),
        (128, true) => matches.push("SHA-512"),
        (16, true) => matches.push("CRC32 (hex) / MySQL3 / LM"),
        (32, false) => matches.push("NTLM (or non-hex MD5)"),
        (16, false) => matches.push("CRC32 (raw)"),
        (64, false) => matches.push("Base64 blob (may be a sha256 variant)"),
        _ => {}
    }
    if matches.is_empty() {
        matches.push("unknown hash format");
    }
    matches.join(", ")
}
