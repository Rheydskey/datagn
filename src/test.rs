#[test]
pub fn test() {
    use crate::config::DatabaseConfig;
    use crate::database::DatabaseType;
    use logger::{error, info};
    use sqlx::Row;

    async_std::task::block_on(async {
        let mut e = DatabaseConfig {
            database_type: DatabaseType::Sqlite,
            ip: String::from("./db.sql"),
            port: String::from("3306"),
            user: String::from("root"),
            password: String::from("root"),
            database: None,
        }
        .to_datapool()
        .await;

        match e
            .execute(
                "CREATE TABLE IF NOT EXISTS User (
            name TEXT NOT NULL,
            password TEXT NOT NULL
        );",
            )
            .await
        {
            Ok(_) => info("Working"),
            Err(e) => error(e),
        };
        e.execute("INSERT INTO User(name, password) VALUES('Datagn','Password')")
            .await
            .expect("Error");
        match e.execute_and_fetch_one("SELECT * FROM User").await {
            Ok(result) => {
                let name: String = result.try_get("name").expect("Error");
                println!("{}", name);
            }
            Err(e) => error(e),
        };
    });
}
