use chrono::Utc;
use minreq;

pub fn get(event: &str, uri: &str) -> String {
    let url = format!("http://127.0.0.1:3320/{}/{}", event, uri);
    let res = minreq::get(url).send().unwrap();

    return res.as_str().unwrap().to_string();
}

pub fn post(event: &str, uri: &str, body: &str) -> String {
    let url = format!("http://127.0.0.1:3320/{}/{}", event, uri);
    let res = minreq::post(url)
        .with_body(body).send().unwrap();

    return res.as_str().unwrap().to_string();
}

pub fn delete(event: &str, uri: &str) -> String {
    let url = format!("http://127.0.0.1:3320/{}/{}", event, uri);
    let res = minreq::delete(url).send().unwrap();

    return res.as_str().unwrap().to_string();
}

pub fn id() -> String {
    format!("t{}", Utc::now().timestamp())
}
