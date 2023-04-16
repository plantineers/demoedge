fn main() {
    println!("Hello, plants!");
    let my_data = SensorData{id:"s1".to_string(),sensor_data: Vec::new()};
}
//no frcking clue what the data to send looks like / what type it has
 fn send_data(url:String,data:SensorData){
    let client = reqwest::Client::new();
    let res = client.post(url)
        // .body(data) //this or json
        .json(data) //creates json from data
        .send();
        // .await?;
}

struct SensorData{
    id: String,
    sensor_data: Vec<Data>
}

pub struct Data{
 sensor_type: String,
 value: f32
}