// use hex::FromHex;

pub fn is_hex_repr(s: &str) -> bool {
    s.chars().all(|x| "0123456789abcdefABCDEF ".contains(x))
}

pub fn is_all_in(s: &str, l: &[char]) -> bool {
    s.chars().all(|c| l.contains(&c))
}
