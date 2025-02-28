use reqwest;
use serde_json::json;
use std::env;
use dotenv::dotenv;
use std::error::Error;

pub fn generate_fix(vulnerability: &str, code_snippet: &str) -> Result<(String, String), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    let client = reqwest::blocking::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let prompt = format!(
        "You are an expert security engineer. Analyze this vulnerability and provide a fix. Explain why the fix is secure.\n\n\
        Vulnerability: {}\n\n\
        Code Snippet:\n{}\n\n\
        Respond with JSON in this format:\n\
        {{ \"fixed_code\": \"<FIXED_CODE>\", \"explanation\": \"<EXPLANATION>\" }}",
        vulnerability, code_snippet
    );

    let request_body = json!({
        "model": "gpt-4",
        "messages": [{"role": "user", "content": prompt}]
    });

    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()?
        .json::<serde_json::Value>()?;

    let fixed_code = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No fix generated.")
        .to_string();

    let explanation = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No explanation provided.")
        .to_string();

    Ok((fixed_code, explanation))
}
