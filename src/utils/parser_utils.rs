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

// macros for parsing
#[macro_export]
macro_rules! parse_field {
    ($interval:expr, $field:ident, f64) => {
        parse_to_f64(&$interval.$field, stringify!($field))?
    };
    ($interval:expr, $field:ident, i64) => {
        parse_to_i64(&$interval.$field, stringify!($field))?
    };
}
