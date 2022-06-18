use mongodb::{bson::doc, Client, Database};
use std::env;

use crate::structure::user::User;

pub async fn connect_db() -> DataBase {
    let database_url = if let Ok(database_url) = env::var("DATABASE_URL") {
        database_url
    } else {
        println!("Cannot read the env var DATABASE_URL, use default value: \"localhost\"");
        "mongodb://dev_test:dev_test@127.0.0.1:27017/test?appName=test,w=majority".to_string()
    };

    let database_name = if let Ok(database_name) = env::var("DATABASE_NAME") {
        database_name
    } else {
        println!("Cannot read the env var DATABASE_NAME, use default value: \"test\"");
        "test".to_string()
    };

    let db = Client::with_uri_str(database_url)
        .await
        .unwrap()
        .database(&database_name);

    db.collection::<User>("user")
        .delete_many(doc! {}, None)
        .await
        .unwrap();

    println!("Database Connect Success");
    DataBase { db }
}

pub struct DataBase {
    pub db: Database,
}
