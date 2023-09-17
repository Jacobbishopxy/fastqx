//! file: conn.rs
//! author: Jacob Xie
//! date: 2023/09/09 18:51:43 Saturday
//! brief:

use anyhow::{anyhow, Result};
use pyo3::pyclass;
use sea_query::{
    InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder,
    TableCreateStatement, TableDropStatement,
};
use sqlx::mysql::{MySql, MySqlRow};
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::sqlite::{Sqlite, SqliteRow};
use sqlx::{FromRow, Pool};

use super::adt::*;
use super::mssql::{PoolConnectionMsSql, PoolMsSql};

// ================================================================================================
// ConnectorStatement
//
// A struct who derived `FqxSchema` auto impl this trait, see `tests/sql_sttm_derive.rs`
// ================================================================================================

pub trait ConnectorStatement
where
    Self: Send + Unpin,
    Self: for<'r> FromRow<'r, MySqlRow>,
    Self: for<'r> FromRow<'r, PgRow>,
    Self: for<'r> FromRow<'r, SqliteRow>,
{
    fn create_table() -> TableCreateStatement;

    fn drop_table() -> TableDropStatement;

    fn insert(data: Vec<Self>) -> Result<InsertStatement>;
}

// ================================================================================================
// FqxPool & FqxPoolConnection
// ================================================================================================

#[derive(Debug, Clone)]
pub enum FqxPool {
    M(Pool<MySql>),
    P(Pool<Postgres>),
    S(Pool<Sqlite>),
    Q(PoolMsSql),
}

impl FqxPool {
    pub fn get_m(&self) -> Option<&Pool<MySql>> {
        match self {
            FqxPool::M(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_p(&self) -> Option<&Pool<Postgres>> {
        match self {
            FqxPool::P(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_s(&self) -> Option<&Pool<Sqlite>> {
        match self {
            FqxPool::S(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_q(&self) -> Option<&PoolMsSql> {
        match self {
            FqxPool::Q(p) => Some(p),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum FqxPoolConnection {
    M(PoolConnection<MySql>),
    P(PoolConnection<Postgres>),
    S(PoolConnection<Sqlite>),
    Q(PoolConnectionMsSql),
}

impl FqxPoolConnection {
    pub fn get_m(&self) -> Option<&PoolConnection<MySql>> {
        match self {
            FqxPoolConnection::M(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_p(&self) -> Option<&PoolConnection<Postgres>> {
        match self {
            FqxPoolConnection::P(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_s(&self) -> Option<&PoolConnection<Sqlite>> {
        match self {
            FqxPoolConnection::S(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_q(&self) -> Option<&PoolConnectionMsSql> {
        match self {
            FqxPoolConnection::Q(p) => Some(p),
            _ => None,
        }
    }
}

// ================================================================================================
// Connector
// ================================================================================================

#[pyclass]
#[derive(Debug, Clone)]
pub struct Connector {
    conn_str: String,
    db: FqxPool,
}

impl Connector {
    pub async fn new<S: Into<String>>(conn_str: S) -> Result<Self> {
        let conn_str = conn_str.into();
        let db = match &conn_str.split_once("://") {
            Some((MYSQL, _)) => FqxPool::M(Pool::<MySql>::connect(&conn_str).await?),
            Some((POSTGRES, _)) => FqxPool::P(Pool::<Postgres>::connect(&conn_str).await?),
            Some((SQLITE, _)) => FqxPool::S(Pool::<Sqlite>::connect(&conn_str).await?),
            Some((MSSQL, _)) => FqxPool::Q(PoolMsSql::new_from_str(&conn_str).await?),
            _ => {
                return Err(anyhow!(
                    "driver not found, check your connect string: {}",
                    &conn_str
                ))
            }
        };

        Ok(Self {
            conn_str: conn_str.into(),
            db,
        })
    }

    pub fn conn_str(&self) -> &str {
        &self.conn_str
    }

    pub fn db(&self) -> &FqxPool {
        &self.db
    }

    pub async fn acquire(&self) -> Result<FqxPoolConnection> {
        match self.db() {
            FqxPool::M(p) => {
                let c = p.acquire().await?;
                Ok(FqxPoolConnection::M(c))
            }
            FqxPool::P(p) => {
                let c = p.acquire().await?;
                Ok(FqxPoolConnection::P(c))
            }
            FqxPool::S(p) => {
                let c = p.acquire().await?;
                Ok(FqxPoolConnection::S(c))
            }
            FqxPool::Q(p) => {
                let c = p.acquire().await?;
                Ok(FqxPoolConnection::Q(c))
            }
        }
    }

    pub async fn close(&self) -> Result<&Self> {
        match self.db() {
            FqxPool::M(p) => p.close().await,
            FqxPool::P(p) => p.close().await,
            FqxPool::S(p) => p.close().await,
            FqxPool::Q(p) => p.close().await?,
        };

        Ok(self)
    }

    pub fn is_close(&self) -> bool {
        match self.db() {
            FqxPool::M(p) => p.is_closed(),
            FqxPool::P(p) => p.is_closed(),
            FqxPool::S(p) => p.is_closed(),
            FqxPool::Q(p) => p.is_closed(),
        }
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        match self.db() {
            FqxPool::M(p) => {
                sqlx::query(sql).execute(p).await?;
            }
            FqxPool::P(p) => {
                sqlx::query(sql).execute(p).await?;
            }
            FqxPool::S(p) => {
                sqlx::query(sql).execute(p).await?;
            }
            FqxPool::Q(p) => {
                p.execute(sql).await?;
            }
        };

        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: ConnectorStatement,
    {
        let res = match self.db() {
            FqxPool::M(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
            FqxPool::P(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
            FqxPool::S(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
            FqxPool::Q(_) => todo!(),
        };

        Ok(res)
    }

    pub async fn save<R>(&self, data: Vec<R>, mode: SaveMode) -> Result<()>
    where
        R: ConnectorStatement,
    {
        let insert_data = R::insert(data)?;
        let is = match self.db() {
            FqxPool::M(_) => insert_data.to_string(MysqlQueryBuilder),
            FqxPool::P(_) => insert_data.to_string(PostgresQueryBuilder),
            FqxPool::S(_) => insert_data.to_string(SqliteQueryBuilder),
            FqxPool::Q(_) => todo!(),
        };

        match mode {
            SaveMode::Override => {
                let drop_table = R::drop_table();
                let create_table = R::create_table();
                let (dt, ct) = match self.db() {
                    FqxPool::M(_) => (
                        drop_table.to_string(MysqlQueryBuilder),
                        create_table.to_string(MysqlQueryBuilder),
                    ),
                    FqxPool::P(_) => (
                        drop_table.to_string(PostgresQueryBuilder),
                        create_table.to_string(PostgresQueryBuilder),
                    ),
                    FqxPool::S(_) => (
                        drop_table.to_string(SqliteQueryBuilder),
                        create_table.to_string(SqliteQueryBuilder),
                    ),
                    FqxPool::Q(_) => todo!(),
                };

                let _ = self.execute(&dt).await;
                self.execute(&ct).await?;
                self.execute(&is).await?;
            }
            SaveMode::Append => {
                self.execute(&is).await?;
            }
        }

        Ok(())
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_db {
    use crate::prelude::rowprocess::FqxSqlRowProcessor;

    use super::*;
    use futures::TryStreamExt;

    const PG_URL: &str = "postgres://dev:devpass@localhost:5437/dev";

    #[tokio::test]
    async fn test_conn() {
        let c = Connector::new(PG_URL).await.unwrap();

        c.acquire().await.unwrap();
    }

    #[tokio::test]
    async fn fetch_dyn2() {
        let conn = Connector::new(PG_URL).await.unwrap();

        let sql = "select * from users";
        let pool = conn.db().get_p().unwrap();

        let mut proc = FqxSqlRowProcessor::new();

        let stream = sqlx::query(sql)
            .try_map(|r| proc.process_sqlx_row(r))
            .fetch(pool);

        let res = stream.try_collect::<Vec<_>>().await.unwrap();

        println!("{:?}", res);
    }
}
