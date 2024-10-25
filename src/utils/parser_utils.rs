pub fn parse_to_f64(value: &str, field_name: &str) -> Result<f64, Box<dyn std::error::Error>> {
    value.parse::<f64>().map_err(|_| {
        format!("Failed to parse {} as f64", field_name).into()
    })
}

pub fn parse_to_i64(value: &str, field_name: &str) -> Result<i64, Box<dyn std::error::Error>> {
    value.parse::<i64>().map_err(|_| {
        format!("Failed to parse {} as i64", field_name).into()
    })
}
