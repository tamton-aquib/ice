use anyhow::Result;
use reqwest::Url;

pub struct Cracker {
    pub query: String,
    pub result: String,
}

impl Cracker {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            result: String::new(),
        }
    }
    pub async fn get(self: &Self) -> Result<Self> {
        const URL: &str = "https://api.dehash.lt/api.php?search=";
        let url = Url::parse(&*format!("{}{}", URL, self.query))?;
        let res = reqwest::get(url).await?.text().await?;
        let last = res.lines().last().unwrap();

        Ok(Cracker {
            query: self.query.clone(),
            result: last.to_string(),
        })
    }
}

/// Crack hashes using the dehash api.
/// Returns a `Result`
/// * `query`: &str (query string)
pub async fn start_cracker(query: &str) -> Result<String> {
    let res = Cracker::new(query).get().await?;

    let nice = if res.result.find(':') != None {
        res.result.split(':').last().unwrap().to_string()
    } else {
        String::from("No Password Found!")
    };

    Ok(nice)
}

