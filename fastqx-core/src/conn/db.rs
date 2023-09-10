//! file: db.rs
//! author: Jacob Xie
//! date: 2023/09/09 18:51:43 Saturday
//! brief:

use anyhow::{anyhow, Result};
use sea_query::{InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};
use sea_query::{TableCreateStatement, TableDropStatement};
use sqlx::mysql::{MySql, MySqlRow};
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::sqlite::{Sqlite, SqliteRow};
use sqlx::{FromRow, Pool};

// ================================================================================================
// Const
// ================================================================================================

const MYSQL: &str = "mysql";
const POSTGRES: &str = "postgres";
const SQLITE: &str = "sqlite";

// ================================================================================================
// ConnectorStatement
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
// Connector<D>
// ================================================================================================

#[derive(Debug, Clone)]
pub enum FqxPool {
    M(Pool<MySql>),
    P(Pool<Postgres>),
    S(Pool<Sqlite>),
}

#[derive(Debug)]
pub enum FqxPoolConnection {
    M(PoolConnection<MySql>),
    P(PoolConnection<Postgres>),
    S(PoolConnection<Sqlite>),
}

#[derive(Debug, Clone)]
pub struct Connector {
    conn_str: String,
    db: FqxPool,
}

impl Connector {
    pub fn new<S: Into<String>>(conn_str: S) -> Result<Self> {
        let conn_str = conn_str.into();
        let db = match &conn_str.split_once("://") {
            Some((MYSQL, _)) => FqxPool::M(Pool::<MySql>::connect_lazy(&conn_str)?),
            Some((POSTGRES, _)) => FqxPool::P(Pool::<Postgres>::connect_lazy(&conn_str)?),
            Some((SQLITE, _)) => FqxPool::S(Pool::<Sqlite>::connect_lazy(&conn_str)?),
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

    pub async fn acquire(&self) -> Result<FqxPoolConnection> {
        match &self.db {
            FqxPool::M(p) => {
                let res = p.acquire().await?;
                Ok(FqxPoolConnection::M(res))
            }
            FqxPool::P(p) => {
                let res = p.acquire().await?;
                Ok(FqxPoolConnection::P(res))
            }
            FqxPool::S(p) => {
                let res = p.acquire().await?;
                Ok(FqxPoolConnection::S(res))
            }
        }
    }

    pub async fn close(&self) -> Result<&Self> {
        match &self.db {
            FqxPool::M(p) => p.close().await,
            FqxPool::P(p) => p.close().await,
            FqxPool::S(p) => p.close().await,
        };

        Ok(self)
    }

    pub fn is_close(&self) -> bool {
        match &self.db {
            FqxPool::M(p) => p.is_closed(),
            FqxPool::P(p) => p.is_closed(),
            FqxPool::S(p) => p.is_closed(),
        }
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        match &self.db {
            FqxPool::M(p) => {
                sqlx::query(sql).execute(p).await?;
            }
            FqxPool::P(p) => {
                sqlx::query(sql).execute(p).await?;
            }
            FqxPool::S(p) => {
                sqlx::query(sql).execute(p).await?;
            }
        };

        Ok(())
    }

    pub async fn fetch_all<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: ConnectorStatement,
    {
        let res = match &self.db {
            FqxPool::M(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
            FqxPool::P(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
            FqxPool::S(p) => sqlx::query_as::<_, R>(sql).fetch_all(p).await?,
        };

        Ok(res)
    }

    pub async fn fetch_one<R>(&self, sql: &str) -> Result<R>
    where
        R: ConnectorStatement,
    {
        let res = match &self.db {
            FqxPool::M(p) => sqlx::query_as::<_, R>(sql).fetch_one(p).await?,
            FqxPool::P(p) => sqlx::query_as::<_, R>(sql).fetch_one(p).await?,
            FqxPool::S(p) => sqlx::query_as::<_, R>(sql).fetch_one(p).await?,
        };

        Ok(res)
    }

    pub async fn fetch_optional<R>(&self, sql: &str) -> Result<Option<R>>
    where
        R: ConnectorStatement,
    {
        let res = match &self.db {
            FqxPool::M(p) => sqlx::query_as::<_, R>(sql).fetch_optional(p).await?,
            FqxPool::P(p) => sqlx::query_as::<_, R>(sql).fetch_optional(p).await?,
            FqxPool::S(p) => sqlx::query_as::<_, R>(sql).fetch_optional(p).await?,
        };

        Ok(res)
    }

    pub async fn save<R>(&self, data: Vec<R>, mode: SaveMode) -> Result<()>
    where
        R: ConnectorStatement,
    {
        let insert_data = R::insert(data)?;
        let is = match &self.db {
            FqxPool::M(_) => insert_data.to_string(MysqlQueryBuilder),
            FqxPool::P(_) => insert_data.to_string(PostgresQueryBuilder),
            FqxPool::S(_) => insert_data.to_string(SqliteQueryBuilder),
        };

        match mode {
            SaveMode::Override => {
                let drop_table = R::drop_table();
                let create_table = R::create_table();
                let (dt, ct) = match &self.db {
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

pub enum SaveMode {
    Override,
    Append,
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_db {
    use super::*;

    const PG_URL: &str = "postgres://dev:devpass@localhost:5437/dev";

    #[tokio::test]
    async fn test_conn() {
        let c = Connector::new(PG_URL).unwrap();

        c.acquire().await.unwrap();
    }
}
