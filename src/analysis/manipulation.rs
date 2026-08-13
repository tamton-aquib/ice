pub fn lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn length(s: &str) -> String {
    s.len().to_string()
}

pub fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

pub fn sort(s: &str) -> String {
    let mut lines: Vec<&str> = s.lines().collect();
    lines.sort();
    lines.join("\n")
}

pub fn unique(s: &str) -> String {
    let mut seen = std::collections::HashSet::new();
    s.lines()
        .filter(|l| seen.insert(*l))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn title(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(first) => {
                    let rest: String = chars.as_str().to_lowercase();
                    first.to_uppercase().collect::<String>() + &rest
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

//  pub fn unique(s: &str) -> String
