use std::env;

use dotenv::dotenv;
use mongodb::Client;

pub struct DataBase{
    
}

impl DataBase{
    pub async fn init() -> Self{
        dotenv().ok();

        let uri = env::var("DB").unwrap();

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("token-metrics");

        DataBase{
            
        }
    }
}