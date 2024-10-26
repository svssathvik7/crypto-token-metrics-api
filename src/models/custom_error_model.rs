use mongodb::error::Error as mongoError;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum CustomError {
    InvalidInput(String),
    DatabaseError(String),
    StandardError(String)
}
impl From<mongoError> for CustomError {
    fn from(err: mongoError) -> Self {
        CustomError::DatabaseError(err.to_string())
    }
}