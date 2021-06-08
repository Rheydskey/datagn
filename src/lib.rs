#[cfg(not(any(feature = "sqlite", feature = "mysql", feature = "postgres",)))]
compile_error!("one of the features ['sqlite', 'mysql', 'postgres'] must be enabled");

pub mod config;
pub mod database;
pub mod test;
pub mod utils;

#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
use sqlx::{any::AnyRow, Executor, Pool};

#[cfg(feature = "mysql")]
use sqlx::{MySql, mysql::MySqlRow};

#[cfg(feature = "postgres")]
use sqlx::{Postgres, postgres::PgRow};
#[cfg(feature = "sqlite")]
use sqlx::{Sqlite,  sqlite::SqliteRow};
#[cfg(feature = "mssql")]
use sqlx::{Mssql, sqlx::mssql::MssqlRow};

#[derive(Clone, Debug)]
pub enum DatabasePool {
    #[cfg(feature = "sqlite")]
    Sqlite(Pool<Sqlite>),
    #[cfg(feature = "mysql")]
    Mysql(Pool<MySql>),
    #[cfg(feature = "postgres")]
    Postgre(Pool<Postgres>),
    #[cfg(feature = "mssql")]
    Mssql(Pool<Mssql>)
}

#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
trait ToAnyRows {
    fn to_anyrows(self) -> Vec<AnyRow>;
}

#[cfg(feature = "mysql")]
impl ToAnyRows for Vec<MySqlRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

#[cfg(feature = "postgres")]
impl ToAnyRows for Vec<PgRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

#[cfg(feature = "sqlite")]
impl ToAnyRows for Vec<SqliteRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}

#[cfg(feature = "mssql")]
impl ToAnyRows for Vec<MssqlRow> {
    fn to_anyrows(self) -> Vec<AnyRow> {
        let mut result = Vec::new();
        for i in self {
            result.push(AnyRow::from(i));
        }
        result
    }
}
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
trait ToAnyRow {
    fn to_anyrow(self) -> AnyRow;
}

#[cfg(feature = "mysql")]
impl ToAnyRow for MySqlRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

#[cfg(feature = "postgres")]
impl ToAnyRow for PgRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

#[cfg(feature = "sqlite")]
impl ToAnyRow for SqliteRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

#[cfg(feature = "mssql")]
impl ToAnyRow for MssqlRow {
    fn to_anyrow(self) -> AnyRow {
        AnyRow::from(self)
    }
}

#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
use std::fmt::Display;
#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
use crate::utils::sqlstrip;

#[cfg(any(feature = "sqlite", feature = "mysql", feature = "postgres", feature = "mssql"))]
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.execute(query).await {
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.execute(replaced_query.as_str()).await {
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.fetch_one(query).await {
                Ok(ms_rows) => Ok(ms_rows.to_anyrow()),
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.fetch_one(replaced_query.as_str()).await {
                Ok(ms_rows) => Ok(ms_rows.to_anyrow()),
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.fetch_all(query).await {
                Ok(ms_rows) => Ok(ms_rows.to_anyrows()),
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(ms_row) => Ok(ms_row.to_anyrows()),
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
            #[cfg(feature = "mssql")]
            DatabasePool::Mssql(e) => {
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
