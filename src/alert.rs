use serde_json::json;
use reqwest;
use crate::utils;

pub async fn send_alert(day: i32) {
    let url = std::env::var("ALERT_URL").unwrap();
    println!("url: {}", url);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&json!({
            "title": format!("🌧️It will rain {}", utils::get_day_name(day)),
            "body": "天気予報を確認して、傘を持ってください。",
        }))
        .send()
        .await
        .unwrap();

    println!("alert res: {} {}", res.status(), res.text().await.unwrap());
}