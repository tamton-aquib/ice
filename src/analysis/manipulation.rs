use anyhow::{anyhow, Result};

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

pub fn trim(s: &str) -> String {
    s.trim().to_string()
}

pub fn count(s: &str, needle: &str) -> String {
    s.matches(needle).count().to_string()
}

fn split_words(s: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut prev: Option<char> = None;
    for c in s.chars() {
        if c.is_alphanumeric() {
            if let Some(p) = prev {
                if p.is_ascii_lowercase() && c.is_ascii_uppercase() {
                    words.push(std::mem::take(&mut cur));
                }
            }
            cur.push(c);
        } else if !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
        prev = Some(c);
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

pub fn case(s: &str, style: Option<&str>) -> Result<String> {
    let words = split_words(s);
    let snake = words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("_");
    let kebab = words
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>()
        .join("-");
    let camel = words
        .iter()
        .enumerate()
        .map(|(i, w)| {
            if i == 0 {
                w.to_lowercase()
            } else {
                capitalize_word(w)
            }
        })
        .collect::<String>();
    let pascal = words.iter().map(|w| capitalize_word(w)).collect::<String>();
    match style {
        Some("snake") => Ok(snake),
        Some("kebab") => Ok(kebab),
        Some("camel") => Ok(camel),
        Some("pascal") => Ok(pascal),
        Some(other) => Err(anyhow!(
            "Unknown case style '{}'. Use snake, kebab, camel or pascal",
            other
        )),
        None => Ok(format!(
            "snake: {}\nkebab: {}\ncamel: {}\npascal: {}",
            snake, kebab, camel, pascal
        )),
    }
}

//  pub fn unique(s: &str) -> String
