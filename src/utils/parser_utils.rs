use std::error::Error as stdError;

pub fn parse_to_type<T>(value: &str, field_name: &str) -> Result<T, Box<dyn stdError>>
where
    T: std::str::FromStr,
    T::Err: stdError + 'static,
{
    value.parse::<T>().map_err(|_| {
        format!("Failed to parse {} as {}", field_name, std::any::type_name::<T>()).into()
    })
}


// macros for parsing
#[macro_export]
macro_rules! parse_field {
    ($interval:expr, $field:ident, $type:ty) => {
        crate::utils::parser_utils::parse_to_type::<$type>(&$interval.$field, stringify!($field))?
    };
}
