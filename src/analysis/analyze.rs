use anyhow::{anyhow, Context, Result};
use std::path::Path;

pub fn frequency(s: &str) -> String {
    let mut counts = std::collections::BTreeMap::<char, usize>::new();
    let mut total = 0usize;
    for c in s.chars().filter(|c| c.is_ascii_alphabetic()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        total += 1;
    }
    if total == 0 {
        return String::from("(no letters found)");
    }
    let mut out = String::new();
    for (c, n) in counts {
        out.push_str(&format!(
            "{}: {} ({:.1}%)\n",
            c,
            n,
            (n as f64) / (total as f64) * 100.0
        ));
    }
    out.trim().to_string()
}

pub fn entropy(input: &str) -> Result<String> {
    let data = if Path::new(input).exists() {
        std::fs::read(Path::new(input)).context("Failed to read file")?
    } else {
        input.as_bytes().to_vec()
    };
    if data.is_empty() {
        return Ok(String::from("0.0000 bits/byte (0 bytes)"));
    }
    let mut counts = [0usize; 256];
    for &b in &data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    let h: f64 = counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len;
            -p * p.log2()
        })
        .sum();
    Ok(format!("{:.4} bits/byte ({} bytes)", h, data.len()))
}

pub fn filetype(file: &str) -> Result<String> {
    let data = std::fs::read(Path::new(file)).context("Failed to read file")?;
    Ok(guess_filetype(&data))
}

fn guess_filetype(data: &[u8]) -> String {
    let checks: &[(&[u8], &str)] = &[
        (b"\x89PNG\r\n\x1a\n", "PNG image"),
        (b"\xFF\xD8\xFF", "JPEG image"),
        (b"GIF87a", "GIF image"),
        (b"GIF89a", "GIF image"),
        (b"%PDF-", "PDF document"),
        (b"PK\x03\x04", "ZIP archive (or docx/jar)"),
        (b"\x7fELF", "ELF executable"),
        (b"MZ", "PE executable (Windows)"),
        (b"\x1f\x8b", "gzip compressed"),
        (b"BZh", "bzip2 compressed"),
        (b"\xfd7zXZ\x00", "xz compressed"),
        (b"7z\xbc\xaf\x27\x1c", "7-Zip archive"),
        (b"Rar!\x1a\x07", "RAR archive"),
        (b"\x00asm", "WebAssembly binary"),
        (b"#!", "script (shebang)"),
        (b"SQLite format 3\x00", "SQLite database"),
        (b"\xca\xfe\xba\xbe", "Java class file"),
        (b"\xed\xab\xee\xdb", "RPM package"),
        (b"OggS", "Ogg container (audio/video)"),
        (b"fLaC", "FLAC audio"),
        (b"ID3", "MP3 audio (ID3 tag)"),
        (b"\x1bL", "LZMA compressed"),
        (b"\x1bM", "LZMA compressed"),
    ];
    for (magic, name) in checks {
        if data.starts_with(magic) {
            return name.to_string();
        }
    }
    let ascii = data
        .iter()
        .take(512)
        .all(|&b| b.is_ascii_graphic() || matches!(b, b'\n' | b'\r' | b'\t' | b' '));
    if ascii {
        String::from("ASCII text")
    } else {
        String::from("unknown binary data")
    }
}

pub fn read_binary(file: &str, query: &str) -> Result<String> {
    let data = std::fs::read(Path::new(file)).context("Failed to read file")?;
    if query.is_empty() {
        return Err(anyhow!("Query must not be empty"));
    }
    let q = query.as_bytes();
    if q.len() > data.len() {
        return Ok(String::new());
    }
    let mut hits: Vec<String> = Vec::new();
    let mut start = 0;
    while start + q.len() <= data.len() {
        match data[start..].windows(q.len()).position(|w| w == q) {
            Some(rel) => {
                let pos = start + rel;
                hits.push(format!("0x{:08x}: {}", pos, printable_window(&data, pos, q.len())));
                start = pos + 1;
            }
            None => break,
        }
    }
    Ok(hits.join("\n"))
}

fn printable_window(data: &[u8], pos: usize, len: usize) -> String {
    let from = pos.saturating_sub(16);
    let to = (pos + len + 16).min(data.len());
    data[from..to]
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}
