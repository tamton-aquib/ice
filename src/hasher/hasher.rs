use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn md5(input: &str) -> String {
    if Path::new(input).exists() {
        let mut file = File::open(input).unwrap();
        let mut hasher = Md5::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
        format!("{:x}", hasher.finalize())
    } else {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub fn sha1(input: &str) -> String {
    if Path::new(input).exists() {
        let mut file = File::open(input).unwrap();
        let mut hasher = Sha1::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
        format!("{:x}", hasher.finalize())
    } else {
        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub fn sha256(input: &str) -> String {
    if Path::new(input).exists() {
        let mut file = File::open(input).unwrap();
        let mut hasher = Sha256::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
        format!("{:x}", hasher.finalize())
    } else {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub fn sha512(input: &str) -> String {
    if Path::new(input).exists() {
        let mut file = File::open(input).unwrap();
        let mut hasher = Sha512::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
        format!("{:x}", hasher.finalize())
    } else {
        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

// pub struct Cracker {
//     pub query: String,
//     pub result: String,
// }

// impl Cracker {
//     pub fn new(s: &str) -> Self {
//         Self {
//             query: s.to_string(),
//             result: String::new(),
//         }
//     }
//     pub async fn get(self: &Self) -> Result<Self> {
//         const URL: &str = "https://api.dehash.lt/api.php?search=";
//         let url = Url::parse(&*format!("{}{}", URL, self.query))?;
//         let res = reqwest::get(url).await?.text().await?;
//         let last = res.lines().last().unwrap();
//
//         Ok(Cracker {
//             query: self.query.clone(),
//             result: last.to_string(),
//         })
//     }
// }

// pub async fn start_cracker(s: &str) -> Result<String> {
//     let res = Cracker::new(s).get().await?;
//
//     let nice = if res.result.find(':') != None {
//         res.result.split(':').last().unwrap().to_string()
//     } else {
//         String::from("No Password Found!")
//     };
//
//     Ok(nice)
// }
