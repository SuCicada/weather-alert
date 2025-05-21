// use anyhow::Result;
use chrono::{DateTime, Datelike, Local};
use open_meteo_rs::forecast::ForecastResult;
use serde_json::json;
use std::env;
// use clap::Parser;
// aa
// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     #[arg(short, long)]
//     location: Option<String>,
// }

mod my;

static mut test_rain: bool = false;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "test-rain" {
            unsafe { test_rain = true }
        }
    }
    main1().await;
    // send_alert().await;
}
async fn main1() {
    // let args = Args::parse();
    // let location = args.location.unwrap_or_else(|| String::from("Tokyo"));

    let current_time: DateTime<Local> = Local::now();
    println!("Weather Alert Service");
    println!("Current time: {}", current_time);
    println!("alert url: {}", std::env::var("ALERT_URL").unwrap());
    println!("location: {}", std::env::var("LOCATION").unwrap());
    println!("{}", get_next_day());
    // println!("Location: {}", location);

    unsafe {
        if test_rain {
            println!("test_rain");
            send_alert().await;
            return;
        }
    }

    let mut res = request_weather_data().await;
    // if let Some(hourly) = res.hourly {
    //     for h in hourly {
    //         // println!("{}: {}", h.datetime, h.values);
    //         let datetime = h.datetime;
    //         let rain = h.values.get("rain").unwrap();
    //         let precipitation = h.values.get("precipitation").unwrap();
    //     }
    // }
    if let Some(daily) = res.daily {
        #[rustfmt::skip]
        let precipitation_sum = daily[0].values.get("precipitation_sum").unwrap().value.as_f64().unwrap();
        #[rustfmt::skip]
        let rain_sum = daily[0].values.get("rain_sum").unwrap().value.as_f64().unwrap();
        format!("{:.2} {}", precipitation_sum, rain_sum);

        if precipitation_sum + rain_sum > 0.0 {
            println!("🌧️It will rain tomorrow");
            send_alert().await
        } else {
            println!("☀️It will not rain tomorrow");
        }
    }
}
async fn send_alert() {
    let url = std::env::var("ALERT_URL").unwrap();
    println!("url: {}", url);
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&json!({
            "title": "🌧️It will rain tomorrow",
            "body": "天気予報を確認して、傘を持ってください。",
        }))
        .send()
        .await
        .unwrap();

    println!("alert res: {} {}", res.status(), res.text().await.unwrap());
}
async fn request_weather_data() -> ForecastResult {
    // let url = format!("https://api.open-meteo.com/v1/forecast?latitude=35.544701&longitude=139.686797&hourly=precipitation_probability&timezone=Asia%2FTokyo&start=2025-01-16T09%3A00&end=2025-01-16T23%3A00")
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();
    let lat_lng = std::env::var("LOCATION").unwrap();
    let (lat, lng) = lat_lng
        .split_once(",")
        .map(|(lat, lng)| (lat.parse::<f64>().unwrap(), lng.parse::<f64>().unwrap()))
        .unwrap();
    opts.location = open_meteo_rs::Location { lat, lng };
    let nextday = get_next_day().date_naive();
    // opts.hourly.push("precipitation".into());
    // opts.hourly.push("rain".into());
    opts.daily.push("precipitation_sum".into());
    opts.daily.push("rain_sum".into());

    opts.start_date = Some(nextday);
    opts.end_date = Some(nextday);
    opts.time_zone = Some("Asia/Tokyo".into());

    let res = client.forecast(opts).await.unwrap();
    return res;
}
fn get_next_day() -> DateTime<Local> {
    let today = Local::now();
    return (today + chrono::Duration::days(1));
    // .format("%Y-%m-%d")
    // .to_string();
}
