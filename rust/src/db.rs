use reqwest;
use chrono::Utc;

pub async fn get(event: &str, uri: &str) -> String {
    let url = format!("http://localhost:3320/{}/{}", event, uri);
    let client = reqwest::Client::new();
    let res = client.get(url).send().await.unwrap();

    return res.text().await.unwrap();
}

pub async fn post(event: &str, uri: &str, body: String) -> String {
    let url = format!("http://localhost:3320/{}/{}", event, uri);
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await.unwrap();

    return res.text().await.unwrap();
}

pub async fn delete(event: &str, uri: &str) -> String {
    let url = format!("http://localhost:3320/{}/{}", event, uri);
    let client = reqwest::Client::new();
    let res = client.delete(url).send().await.unwrap();

    return res.text().await.unwrap();
}

pub fn id() -> String {
    format!("t{}", Utc::now().timestamp())
}