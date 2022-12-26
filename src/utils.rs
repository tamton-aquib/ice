// use hex::FromHex;

pub trait Chunkify {
    fn chunkify(self) -> Vec<char>;
}

impl Chunkify for &str {
    fn chunkify(self) -> Vec<char> {
        self.split_whitespace()
            .collect::<String>()
            .chars()
            .collect()
    }
}

pub fn is_hex_repr(s: &str) -> bool {
    s.chars().all(|x| "0123456789abcdefABCDEF ".contains(x))
}

pub fn is_all_in(s: &str, l: &[char]) -> bool {
    s.chars().all(|c| l.contains(&c))
}
