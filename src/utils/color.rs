use std::io::IsTerminal;

fn use_color() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}

fn paint(s: &str, code: &str) -> String {
    if use_color() {
        format!("\x1b[{}m{}\x1b[0m", code, s)
    } else {
        s.to_string()
    }
}

pub fn red(s: &str) -> String {
    paint(s, "31")
}

pub fn green(s: &str) -> String {
    paint(s, "32")
}

pub fn yellow(s: &str) -> String {
    paint(s, "33")
}

pub fn cyan(s: &str) -> String {
    paint(s, "36")
}
