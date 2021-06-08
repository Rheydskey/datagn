use crate::{config::DatabaseConfig, DatabasePool};

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

pub struct Database {
    pub database_type: DatabaseType,
    pub ip: String,
    pub port: i32,
    pub db_name: String,
}

#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum DatabaseType {
    #[cfg(feature = "sqlite")]
    Sqlite,
    #[cfg(feature = "mysql")]
    Mysql,
    #[cfg(feature = "postgres")]
    Postgresql,
    #[cfg(feature = "mssql")]
    Mssql,
}

impl Default for DatabaseType {
    fn default() -> Self {
        match () {
            #[cfg(feature = "sqlite")]
            () => DatabaseType::Sqlite,
            #[cfg(feature = "postgres")]
            () => DatabaseType::Postgresql,
            #[cfg(feature = "mysql")]
            () => DatabaseType::Mysql,
            #[cfg(feature = "mssql")]
            () => DatabaseType::Mssql,
        }
    }
}

impl Database {
    pub fn from(ip: String, port: i32, database_type: DatabaseType, db_name: String) -> Self {
        Self {
            database_type,
            ip,
            port,
            db_name,
        }
    }
    pub async fn get_conn_from_config(config: DatabaseConfig) -> DatabasePool {
        match config.database_type {
            #[cfg(feature = "sqlite")]
            DatabaseType::Sqlite => {
                let e = match sqlx::SqlitePool::connect(config.sqlite_format().as_str()).await {
                    Ok(e) => e,
                    Err(e) => {
                        error(format!("Error : {}", e.as_database_error().unwrap()));
                        panic!()
                    }
                };
                DatabasePool::Sqlite(e)
            }
            #[cfg(feature = "mysql")]
            DatabaseType::Mysql => DatabasePool::Mysql(
                sqlx::MySqlPool::connect(config.mysql_format().as_str())
                    .await
                    .unwrap(),
            ),
            #[cfg(feature = "postgres")]
            DatabaseType::Postgresql => DatabasePool::Postgre(
                sqlx::PgPool::connect(config.postgres_format().as_str())
                    .await
                    .unwrap(),
            ),
            #[cfg(feature = "mssql")]
            DatabaseType::Mssql => DatabasePool::Mssql(
                sqlx::MssqlPool::connect(config.mssql_format().as_str())
                    .await
                    .unwrap(),
            ),
        }
    }
}
