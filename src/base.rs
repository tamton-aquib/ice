use base32;
use base64;
use hex;

pub fn b64(s: &str) -> String {
    match base64::decode(s) {
        Ok(n) => n.iter().map(|i| *i as char).collect(),
        Err(_) => base64::encode(s),
    }
}

// FIX: sometimes gives weird result
pub fn b32(s: &str) -> String {
    match base32::decode(base32::Alphabet::RFC4648 { padding: true }, s) {
        Some(n) => {
            // println!("Decoding...");
            n.iter().map(|i| *i as char).collect::<String>()
        }
        None => base64::encode(s),
    }
}

pub fn hexadecimal(s: &str) -> String {
    if s.chars().all(|x| "0123456789abcdef".contains(x)) {
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
            .collect::<String>()
    } else {
        s.chars().fold(String::new(), |acc, i| {
            format!("{acc}{:03o} ", u32::from(i))
        })
    }
}

pub fn binary(s: &str) -> String {
    if s.chars().all(|x| ['0', '1', ' '].contains(&x)) {
        s.split_whitespace()
            .map(|c| u8::from_str_radix(c, 2).unwrap() as char)
            .collect::<String>()
    } else {
        s.chars().fold(String::new(), |acc, i| {
            format!("{acc}{:08b} ", u32::from(i))
        })
    }
}
