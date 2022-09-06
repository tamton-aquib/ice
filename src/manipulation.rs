pub fn lower(s: &str) -> String {
    return s.to_lowercase().to_string();
}

pub fn upper(s: &str) -> String {
    return s.to_uppercase().to_string();
}

pub fn remove_whitespace(s: &str) -> String {
    return s.split_whitespace().collect();
}

//  pub fn unique(s: &str) -> String
