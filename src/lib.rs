pub mod config;
pub mod database;
pub mod test;
pub mod utils;

use crate::utils::sqlstrip;

use sqlx::{any::AnyRow, mysql::MySqlRow, postgres::PgRow, sqlite::SqliteRow, Executor, Pool};

#[cfg(feature = "mysql")]
use sqlx::MySql;
#[cfg(feature = "sqlite")]
use sqlx::Sqlite;
#[cfg(feature = "postgres")]
use sqlx::Postgres;

use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum DatabasePool {
    #[cfg(feature = "sqlite")]
    Sqlite(Pool<Sqlite>),
    #[cfg(feature = "mysql")]
    Mysql(Pool<MySql>),
    #[cfg(feature = "postgres")]
    Postgre(Pool<Postgres>),
}

trait ToAnyRows {
    fn to_anyrows(self) -> Vec<AnyRow>;
}

impl ToAnyRows for Vec<MySqlRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

impl ToAnyRows for Vec<PgRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

impl ToAnyRows for Vec<SqliteRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

trait ToAnyRow {
    fn to_anyrow(self) -> AnyRow;
}

impl ToAnyRow for MySqlRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

impl ToAnyRow for PgRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

impl ToAnyRow for SqliteRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

impl DatabasePool {
    pub async fn execute(&mut self, query: &str) -> Result<(), String> {
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.execute(query).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.execute(query).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.execute(query).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_with_bind<A: Display>(
        &mut self,
        query: &str,
        bind: &[A],
    ) -> Result<(), String> {
        let mut replaced_query = query.to_string();
        let mut n = 1;
        for tobind in bind {
            replaced_query = replaced_query.replace(
                format!("?{}", n).as_str(),
                format!("'{}'", sqlstrip(tobind.to_string())).as_str(),
            );
            n += 1;
        }
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_and_fetch_one(&mut self, query: &str) -> Result<AnyRow, String> {
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.fetch_one(query).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.fetch_one(query).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.fetch_one(query).await {
                Ok(pg_rows) => Ok(pg_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_and_fetch_one_with_bind<A: Display>(
        &mut self,
        query: &str,
        bind: &[A],
    ) -> Result<AnyRow, String> {
        let mut replaced_query = query.to_string();
        let mut n = 1;
        for tobind in bind {
            replaced_query = replaced_query.replace(
                format!("?{}", n).as_str(),
                format!("'{}'", sqlstrip(tobind.to_string())).as_str(),
            );
            n += 1;
        }
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.fetch_one(replaced_query.as_str()).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.fetch_one(replaced_query.as_str()).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.fetch_one(replaced_query.as_str()).await {
                Ok(pg_rows) => Ok(pg_rows.to_anyrow()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_and_fetch_all(&mut self, query: &str) -> Result<Vec<AnyRow>, String> {
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.fetch_all(query).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.fetch_all(query).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.fetch_all(query).await {
                Ok(pg_rows) => Ok(pg_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_and_fetch_all_with_bind<A: Display>(
        &mut self,
        query: &str,
        bind: &[A],
    ) -> Result<Vec<AnyRow>, String> {
        let mut replaced_query = query.to_string();
        let mut n = 1;
        for tobind in bind {
            replaced_query = replaced_query.replace(
                format!("?{}", n).as_str(),
                format!("'{}'", sqlstrip(tobind.to_string())).as_str(),
            );
            n += 1;
        }
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(pg_rows) => Ok(pg_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn executes(&mut self, query: Vec<&str>) -> Result<(), String> {
        match self {
            #[cfg(feature = "sqlite")]
            DatabasePool::Sqlite(e) => {
                for i in query {
                    match e.execute(i).await {
                        Ok(_) => {}
                        Err(e) => return Err(format!("{:?}", e)),
                    }
                }
                Ok(())
            }
            #[cfg(feature = "mysql")]
            DatabasePool::Mysql(e) => {
                for i in query {
                    match e.execute(i).await {
                        Ok(_) => {}
                        Err(e) => return Err(format!("{:?}", e)),
                    }
                }
                Ok(())
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgre(e) => {
                for i in query {
                    match e.execute(i).await {
                        Ok(_) => {}
                        Err(e) => return Err(format!("{:?}", e)),
                    }
                }
                Ok(())
            }
        }
    }
}
