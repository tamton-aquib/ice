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
    let url = format!("https://factordb.com/api?query={}", s);

    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .build();

    let body = agent
        .get(&url)
        .call()
        .map_err(|e| anyhow::anyhow!("Request to factordb.com failed: {}", e))?
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
