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
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .context("Failed to read file")?;
    let hash = D::digest(&buffer);
    Ok(hash.iter().map(|b| format!("{:02x}", b)).collect())
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
