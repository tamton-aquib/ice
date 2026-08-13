use crate::app::cli::build_cli;

pub fn suggest_subcommand(typed: &str) -> Option<String> {
    let typed = typed.to_lowercase();
    let mut scored: Vec<(usize, String)> = Vec::new();

    for sub in build_cli().get_subcommands() {
        let d = levenshtein(&typed, &sub.get_name().to_lowercase());
        if within_threshold(typed.len(), d) {
            scored.push((d, sub.get_name().to_string()));
        }
        for alias in sub.get_all_aliases() {
            let d = levenshtein(&typed, &alias.to_lowercase());
            if within_threshold(typed.len(), d) && !scored.iter().any(|(_, n)| n == alias) {
                scored.push((d, alias.to_string()));
            }
        }
    }

    scored.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    scored.truncate(5);
    if scored.is_empty() {
        None
    } else {
        Some(
            scored
                .iter()
                .map(|(_, n)| n.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

fn within_threshold(typed_len: usize, dist: usize) -> bool {
    dist <= 1 || (dist <= 2 && typed_len >= 5)
}

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            cur[j + 1] = (prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}
