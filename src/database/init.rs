use async_once::AsyncOnce;
use bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref DB: AsyncOnce<DataBase> = AsyncOnce::new(DataBase::new());
}

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub struct DataBase {
    pub pool: ConnectionPool,
}

impl DataBase {
    async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("Cannot read the env var DATABASE_URL");
        let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)
            .expect("Cannot parse the database setting");
        let pool = Pool::builder()
            .build(manager)
            .await
            .expect("Cannot connect to the database");
        pool.get()
            .await
            .expect("Cannot connect to the database")
            .query(
                "CREATE TABLE users (name varchar(20), password varchar(20))",
                &[],
            )
            .await
            .unwrap();
        Self { pool }
    }
}
