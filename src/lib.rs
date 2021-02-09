pub mod config;
pub mod database;
pub mod test;
pub mod utils;

use crate::utils::sqlstrip;

use sqlx::{
    any::AnyRow, mysql::MySqlRow, postgres::PgRow, sqlite::SqliteRow, Executor, MySql, Pool,
    Postgres, Sqlite,
};

use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum DatabasePool {
    Sqlite(Pool<Sqlite>),
    Mariadb(Pool<MySql>),
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
            DatabasePool::Sqlite(e) => match e.execute(query).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Mariadb(e) => match e.execute(query).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
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
            DatabasePool::Sqlite(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Mariadb(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Postgre(e) => match e.execute(replaced_query.as_str()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn execute_and_fetch_all(&mut self, query: &str) -> Result<Vec<AnyRow>, String> {
        match self {
            DatabasePool::Sqlite(e) => match e.fetch_all(query).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Mariadb(e) => match e.fetch_all(query).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
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
            DatabasePool::Sqlite(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(sqlite_rows) => Ok(sqlite_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Mariadb(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(maria_rows) => Ok(maria_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
            DatabasePool::Postgre(e) => match e.fetch_all(replaced_query.as_str()).await {
                Ok(pg_rows) => Ok(pg_rows.to_anyrows()),
                Err(e) => Err(format!("{:?}", e)),
            },
        }
    }
    pub async fn executes(&mut self, query: Vec<&str>) -> Result<(), String> {
        match self {
            DatabasePool::Sqlite(e) => {
                for i in query {
                    match e.execute(i).await {
                        Ok(_) => {}
                        Err(e) => return Err(format!("{:?}", e)),
                    }
                }
                Ok(())
            }
            DatabasePool::Mariadb(e) => {
                for i in query {
                    match e.execute(i).await {
                        Ok(_) => {}
                        Err(e) => return Err(format!("{:?}", e)),
                    }
                }
                Ok(())
            }
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
