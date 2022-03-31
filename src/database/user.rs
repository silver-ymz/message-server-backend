use bb8_postgres::tokio_postgres::Row;

use super::DataBase;
use crate::structure::user::User;

impl DataBase {
    pub async fn insert_user(&self, item: &User) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();

        if !conn.query_one("SELECT name FROM users where name=$1", &[&item.name.as_str()]).await.unwrap().is_empty() {
            return Err(());
        };

        match conn
            .query_one(
                "INSERT INTO users (name, password) VALUES ($1, $2)",
                &[&item.name.as_str(), &item.password.as_str()],
            )
            .await
        {
            Ok(_row) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub async fn find_user(&self, item: &User) -> Result<(), ()> {
        let conn = self.pool.get().await.unwrap();
        let password = match conn.query_one("SELECT password FROM users where name=$1", &[&item.name.as_str()]).await {
            Ok(row) => row,
            Err(_) => return Err(())
        };
    }
}
