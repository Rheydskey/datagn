use crate::{config::DatabaseConfig, DatabasePool};
use logger::error;

pub struct Database {
    pub database_type: DatabaseType,
    pub ip: String,
    pub port: i32,
    pub db_name: String,
}

#[derive(Debug, Clone, Derive)]
pub enum DatabaseType {
    Sqlite,
    Mysql,
    Postgresql,
}

impl Default for DatabaseType {
    fn default() -> Self {
        Self::Mysql
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
            DatabaseType::Mysql => DatabasePool::Mysql(
                sqlx::MySqlPool::connect(config.mysql_format().as_str())
                    .await
                    .unwrap(),
            ),
            DatabaseType::Postgresql => DatabasePool::Postgre(
                sqlx::PgPool::connect(config.postgres_format().as_str())
                    .await
                    .unwrap(),
            ),
        }
    }
}
