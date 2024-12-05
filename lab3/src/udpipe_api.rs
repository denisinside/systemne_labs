use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://lindat.mff.cuni.cz/services/udpipe/api/";

pub async fn process_text(text: &str, model: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post(&format!("{}/process", BASE_URL))
        .form(&[("data", text), ("model", model), ("tokenizer", ""), ("tagger", ""), ("parser", "")])
        .send().await?;
    let json: Value = response.json().await?;
    Ok(json)
}

pub fn extract_significant_words(result: &Value) -> Vec<String> {
    let mut significant_words = vec![];
    if let Some(result_text) = result["result"].as_str() {
        for line in result_text.lines() {
            if !line.starts_with("#") {
                if let Some(columns) = line.split('\t').nth(2) {
                    significant_words.push(columns.to_string());
                }
            }
        }
    }
    significant_words
}