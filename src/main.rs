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
    println!("This is our serialized data: {}", serialized_data);
    // We can also deserialize data
    let json_data = r#"{
        "id": "funghi_group_1",
        "sensor_data": [
            {
                "type": "temperature",
                "value": 30.0
            },
            {
                "type": "humidity",
                "value": 70.0
            },
            {
                "type": "light",
                "value": 30.0
            }
        ]
    }"#;
    let deserialized_data: SensorData = serde_json::from_str(json_data).unwrap();
    println!("This is our deserialized data: {:?}", deserialized_data);
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
