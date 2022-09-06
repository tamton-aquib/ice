// TODO: mostly sub ciphers.

pub fn a1z26(s: &str) -> String {
    const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";
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

// pub fn substitution(s: String, key: char) -> String {
// s.chars();
// }
