fn main() {
    println!("Hello, plants!");
    let my_data = SensorData{id:"s1".to_string(),sensor_data: Vec::new()};
}
struct SensorData{
    id: String,
    sensor_data: Vec<Data>
}

pub struct Data{
 sensor_type: String,
 value: f32
}