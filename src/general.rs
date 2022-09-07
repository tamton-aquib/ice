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
