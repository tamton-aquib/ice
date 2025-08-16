use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FactorEntry(String, u32);

#[derive(Debug, Deserialize)]
struct Data {
    factors: Vec<FactorEntry>,
}

pub fn factordb(s: &str) -> String {
    let response = ureq::get(&format!("http://factordb.com/api?query={}", s)).call();
    if response.status() != 200 {
        return "No Network?".to_string();
    }

    let res = response.into_string();

    let parsed_data: Data =
        serde_json::from_str(&res.unwrap().trim()).expect("Failed to parse JSON");

    parsed_data
        .factors
        .iter()
        .map(|i| i.0.clone())
        .collect::<Vec<String>>()
        .join(" ")
}
