mod test {
    #[test]
    pub fn test() {
        use crate::config::DatabaseConfig;
        use crate::database::DatabaseType;
        use logger::{error, info};

        async_std::task::block_on(async {
            let mut e = DatabaseConfig {
                database_type: DatabaseType::Mariadb,
                ip: "127.0.0.1".to_string(),
                port: "3306".parse().unwrap(),
                user: "root".to_string(),
                password: "".to_string(),
            }
            .to_datapool()
            .await;

            match e.execute("BEGIN;").await {
                Ok(_) => info("Working"),
                Err(e) => error(e),
            };
        });
    }
}
