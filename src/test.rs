use crate::config::DatabaseConfig;
use crate::DatabaseType;
use logger::{info, error};
#[test]
pub fn test() {
    async_std::task::block_on(
        async {
            let file = std::fs::File::create("./db.sql").unwrap();
            let mut e = DatabaseConfig {
                database_type: DatabaseType::Mariadb,
                ip: "127.0.0.1".to_string(),
                port: "3306".parse().unwrap(),
                user: "root".to_string(),
                password: "Rheydskey123!".to_string()
            }.to_datapool().await;
            match e.execute("BEGIN;").await {
                Ok(e) => info("Working"),
                Err(e) => error(e)
            };
        }
    );

}