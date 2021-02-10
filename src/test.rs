#[test]
pub fn test() {
    use crate::config::DatabaseConfig;
    use crate::database::DatabaseType;
    use logger::{error, info};

    async_std::task::block_on(async {
        let mut e = DatabaseConfig {
            database_type: DatabaseType::Mysql,
            ip: String::from("127.0.0.1"),
            port: String::from("3306"),
            user: String::from("root"),
            password: String::from("Rheydskey123!"),
        }
        .to_datapool()
        .await;

        match e.execute("BEGIN;").await {
            Ok(_) => info("Working"),
            Err(e) => error(e),
        };
    });
}
