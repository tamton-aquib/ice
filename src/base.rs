use crate::utils;
use crate::utils::Chunkify;

pub fn b64(s: &str) -> String {
    match base64::decode(s) {
        Ok(n) => n.iter().map(|i| *i as char).collect(),
        Err(_) => base64::encode(s),
    }
}

// FIX: sometimes gives weird result
pub fn b32(s: &str) -> String {
    match base32::decode(base32::Alphabet::RFC4648 { padding: true }, s) {
        Some(n) => n.iter().map(|i| *i as char).collect(),
        None => base64::encode(s),
    }
}

pub fn hexadecimal(s: &str) -> String {
    if utils::is_hex_repr(s) {
        String::from_utf8(hex::decode(s).unwrap()).unwrap()
    } else {
        s.chars()
            .fold(String::new(), |acc, i| format!("{acc}{:x} ", u32::from(i)))
    }
}

pub fn octal(s: &str) -> String {
    if s.chars().all(|x| "01234567 ".contains(x)) {
        s.split_whitespace()
            .map(|c| u8::from_str_radix(c, 8).unwrap() as char)
            .collect()
    } else {
        s.chars().fold(String::new(), |acc, i| {
            format!("{acc}{:03o} ", u32::from(i))
        })
    }
}

pub fn binary(s: &str) -> String {
    if s.chars().all(|x| ['0', '1', ' '].contains(&x)) {
        s.chunkify()
            .chunks(8)
            .map(|c| u8::from_str_radix(&String::from_iter(c), 2).unwrap() as char)
            .collect()
    } else {
        s.chars().fold(String::new(), |acc, i| {
            format!("{acc}{:08b} ", u32::from(i))
        })
    }
}
