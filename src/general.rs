// TODO: mostly sub ciphers.
const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";

// TODO: cleanup maybe
pub fn atbash(s: &str) -> String {
    s.chars()
        .map(|c| {
            let idx = ALPHABETS.find(c).unwrap_or(27);
            match ALPHABETS.chars().rev().nth(idx) {
                Some(v) => v,
                None => ' ',
            }
        })
        .collect()
}

pub fn ascii(s: &str) -> String {
    if s.chars().all(|x| "0123456789 ".contains(x)) {
        s.split_whitespace()
            .map(|n| n.parse::<u8>().expect("Failed when parsing to range 0-255") as char)
            .collect()
    } else {
        s.chars().map(|i| format!("{}", i as u8)).collect()
    }
}

pub fn a1z26(s: &str) -> String {
    s.replace("-", " ")
        .split_whitespace()
        .map(|num| {
            ALPHABETS
                .chars()
                .nth(num.parse::<u8>().unwrap() as usize - 1)
                .unwrap()
        })
        .collect()
}
