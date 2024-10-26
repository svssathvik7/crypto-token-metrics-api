use std::error::Error as stdError;

use mongodb::bson::Bson;

pub fn parse_to_type<T>(value: &str, field_name: &str) -> Result<T, Box<dyn stdError>>
where
    T: std::str::FromStr,
    T::Err: stdError + 'static,
{
    value.parse::<T>().map_err(|_| {
        format!("Failed to parse {} as {}", field_name, std::any::type_name::<T>()).into()
    })
}

pub fn subtract_bson_values(bson_value_a: &Bson, bson_value_b: &Bson) -> f64 {
    // Attempt to convert both Bson values to f64
    let value_a = bson_value_a.as_f64().unwrap_or(0.0);
    let value_b = bson_value_b.as_f64().unwrap_or(0.0);
    
    value_a-value_b
}


// macros for parsing
#[macro_export]
macro_rules! parse_field {
    ($interval:expr, $field:ident, $type:ty) => {
        crate::utils::parser_utils::parse_to_type::<$type>(&$interval.$field, stringify!($field))?
    };
}
