use reqwest::get;
use serde::{Deserialize, Serialize};
use std::env::args;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let url = args().nth(1).expect("Please input a url");
    let (temp, humidity) = get_weather_data(&args().nth(2).expect("Please input an API key")).await;
    let data = SensorData {
        id: "test_sensor".to_string(),
        sensor_data: vec![
            Data {
                r#type: "temperature".to_string(),
                value: f32::from_str(&temp).unwrap(),
            },
            Data {
                r#type: "humidity".to_string(),
                value: f32::from_str(&humidity).unwrap(),
            },
        ],
    };
    println!("Sending Data: \n{:?}", data);
    println!("{}", send_data(url, data).await);
}

async fn get_weather_data(api_key: &str) -> (String, String) {
    let weather_data = get(
        "https://api.openweathermap.org/data/2.5/weather?lat=49.487457&lon=8.466040&appid=API_KEY&units=metric"
            .replace("API_KEY", api_key),
    )
        .await
        .unwrap()
        .text()
        .await.unwrap();
    // Get "temp":
    let temp = weather_data
        .as_str()
        .split(r#""temp":"#)
        .nth(1)
        .unwrap()
        .split(",")
        .next()
        .unwrap();
    // Get "humidity":
    let humidity = weather_data
        .as_str()
        .split(r#""humidity":"#)
        .nth(1)
        .unwrap()
        .split("}")
        .next()
        .unwrap();
    (temp.into(), humidity.into())
}

async fn send_data(url: String, data: SensorData) -> reqwest::StatusCode {
    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&data) //creates json from data
        .send()
        // We need to "await" this since we are using asynchronous code, we could also do this in a
        // blocking way but non-blocking is more performant
        .await
        .unwrap()
        .status()
}
#[derive(Debug, Serialize, Deserialize)]
struct SensorData {
    id: String,
    sensor_data: Vec<Data>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    r#type: String,
    value: f32,
}
