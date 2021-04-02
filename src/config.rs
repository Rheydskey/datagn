use crate::database::Database;
use crate::database::DatabaseType;
use crate::DatabasePool;
#[derive(Debug, Default, Clone)]
pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub ip: String,
    pub port: String,
    pub user: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let database_type = match () {
            #[cfg(feature = "sqlite")]
            () => DatabaseType::Sqlite,
            #[cfg(feature = "mysql")]
            () => DatabaseType::Mysql,
            #[cfg(feature = "postgres")]
            () => DatabaseType::Postgresql,
            #[cfg(feature = "mssql")]
            () => DatabaseType::Mssql,
        };
        DatabaseConfig {
            database_type,
            ip: String::from("127.0.0.1"),
            port: String::from("55555"),
            user: String::from("root"),
            password: String::from("PASSWORD HERE"),
        }
    }
    #[cfg(feature = "mysql")]
    pub fn mysql_format(&self) -> String {
        format!("mysql://{}:{}@{}", self.user, self.password, self.ip)
    }
    #[cfg(feature = "sqlite")]
    pub fn sqlite_format(&self) -> String {
        format!("./{}", self.ip)
    }
    #[cfg(feature = "postgres")]
    pub fn postgres_format(&self) -> String {
        format!("postgres://{}:{}@{}", self.user, self.password, self.ip)
    }
    #[cfg(feature = "mssql")]
    pub fn mssql_format(&self) -> String {
        format!("mssql://{}:{}@{}", self.user, self.password, self.ip)
    }
    pub async fn to_datapool(&self) -> DatabasePool {
        let e = self.clone();
        Database::get_conn_from_config(e).await
    }
}
