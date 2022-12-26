pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn remove_whitespace(s: &str) -> String {
    return s.split_whitespace().collect();
}

//  pub fn unique(s: &str) -> String
