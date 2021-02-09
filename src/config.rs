use crate::database::Database;
use crate::database::DatabaseType;
use crate::DatabasePool;
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub ip: String,
    pub port: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig {
            database_type: DatabaseType::Sqlite,
            ip: String::from("127.0.0.1"),
            port: String::from("55555"),
            user: String::from("root"),
            password: String::from("PASSWORD HERE"),
        }
    }
    pub fn mariadb_format(&self) -> String {
        format!("mysql://{}:{}@{}", self.user, self.password, self.ip)
    }
    pub fn sqlite_format(&self) -> String {
        format!("./{}", self.ip)
    }
    pub fn postgres_format(&self) -> String {
        format!("postgres://{}:{}@{}", self.user, self.password, self.ip)
    }
    pub async fn to_datapool(&self) -> DatabasePool {
        let e = self.clone();
        Database::get_conn_from_config(e).await
    }
}
