use reqwest;
use serde_json::json;
use std::env;
use dotenv::dotenv;
use std::error::Error;

pub fn train_ai(fixed_code: &str, original_code: &str, vulnerability: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    let training_data = json!({
        "examples": [
            {
                "input": format!(
                    "Fix the following security vulnerability:\n\nVulnerability Type: {}\n\n{}",
                    vulnerability, original_code
                ),
                "output": fixed_code
            }
        ]
    });

    let client = reqwest::blocking::Client::new();
    let url = "https://api.openai.com/v1/fine-tunes";

    client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&training_data)
        .send()?;

    Ok(())
}
