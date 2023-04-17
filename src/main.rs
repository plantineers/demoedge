use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, plants!");
    let my_data = SensorData {
        id: "s1".to_string(),
        sensor_data: vec![
            Data {
                r#type: "temperature".to_string(),
                value: 23.0,
            },
            Data {
                r#type: "humidity".to_string(),
                value: 45.0,
            },
        ],
    };
    let serialized_data = serde_json::to_string(&my_data).unwrap();
    
}
//no frcking clue what the data to send looks like / what type it has
async fn send_data(url:String,data:SensorData){
    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&data) //creates json from data
        .send()
        // We need to "await" this since we are using asynchronous code, we could also do this in a blocking way
        // but non-blocking is more performant
        .await;
}
#[derive(Debug, Serialize, Deserialize)]
struct SensorData{
    id: String,
    sensor_data: Vec<Data>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    r#type: String,
    value: f32,
}
