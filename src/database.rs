use sqlx::pool::Pool;
use sqlx::sqlite::Sqlite;

pub struct Database {
    pub db: Pool<Sqlite>,
}

pub struct Settings {
    pub databasename: String,
}

#[derive(Debug)]
pub struct DatabaseError;

impl Database {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection = Pool::connect(&configuration.databasename)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&connection)
            .await
            .expect("Failed to run migrations");

        Ok(Self { db: connection })
    }
}
