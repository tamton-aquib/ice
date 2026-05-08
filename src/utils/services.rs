use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FactorEntry(String, u32);

#[derive(Debug, Deserialize)]
struct Data {
    factors: Vec<FactorEntry>,
}

pub fn factordb(s: &str) -> Result<String> {
    let url = format!("http://factordb.com/api?query={}", s);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut req = ureq::get(&url);
        req.timeout(Duration::from_secs(10));
        req.call()
    }))
    .map_err(|_| anyhow::anyhow!("Request to factordb.com failed (connection error)"))?;

    let body = result
        .into_string()
        .map_err(|e| anyhow::anyhow!("Failed to read response from factordb: {}", e))?;

    let parsed: Data = serde_json::from_str(&body)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON from factordb: {}", e))?;

    Ok(parsed
        .factors
        .iter()
        .map(|i| i.0.clone())
        .collect::<Vec<String>>()
        .join(" "))
}
