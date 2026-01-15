use std::collections::HashMap;

use reqwest;
use serde_json::{json, Value};

pub struct CreateUniversityResponse {
    success: bool,
    password: String,
}

pub async fn create_university_request(
    login: String,
    mail: String,
    name: String,
) -> Result<Value, reqwest::Error> {
    let mut body = HashMap::new();
    body.insert("login", login);
    body.insert("mail", mail);
    body.insert("name", name);
    let url = "http://localhost:8000/create/university";
    let client = reqwest::Client::new();
    let body = client.post(url).json(&body).send().await?.json().await?;
    Ok(body)
}
pub async fn create_company_request() -> Result<Value, reqwest::Error> {
    let payload = json!({
        "test":"test"
    });
    let url = "urlquiseraremplie";
    let body = reqwest::get(url).await?.json().await?;
    Ok(body)
}
