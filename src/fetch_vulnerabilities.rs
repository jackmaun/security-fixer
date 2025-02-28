use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SecurityIssue {
    pub component: String,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct PagingInfo {
    #[serde(rename = "pageIndex")]
    pub page_index: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
struct SonarResponse {
    pub hotspots: Vec<SecurityIssue>,
    pub paging: PagingInfo,
}

pub fn fetch_vulnerabilities() -> Result<Vec<SecurityIssue>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let sonar_url = std::env::var("SONAR_URL")?;
    let sonar_token = std::env::var("SONAR_TOKEN")?;

    let url = format!("{}/api/hotspots/search?projectKey=WebGoat", sonar_url);

    let response_text = reqwest::blocking::Client::new()
        .get(&url)
        .basic_auth(sonar_token, Some("")) 
        .send()?
        .text()?;

    println!("Raw JSON Response: {}", response_text);  // debugging json response

    if response_text.trim().is_empty() {
        return Err("[-] SonarQube API returned an empty response".into());
    }

    let response: SonarResponse = serde_json::from_str(&response_text)?;

    println!(
        "SonarQube Report: page {} of {}, total issues: {}",
        response.paging.page_index, response.paging.page_size, response.paging.total
    );

    if response.hotspots.is_empty() {
        println!("[!] No security vulnerabilities found in WebGoat.");
    }

    Ok(response.hotspots)
}
