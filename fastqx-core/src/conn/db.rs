//! file: db.rs
//! author: Jacob Xie
//! date: 2023/09/09 18:51:43 Saturday
//! brief:

use anyhow::{anyhow, Result};
use sea_query::{InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};
use sea_query::{TableCreateStatement, TableDropStatement};
use sqlx::mysql::{MySql, MySqlRow};
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

#[derive(Debug, Clone)]
pub struct Connector {
    conn_str: String,
    db: Option<FqxPool>,
}

macro_rules! guard {
    ($s:expr) => {
        if $s.db.is_none() {
            return Err(anyhow!("db not connect"));
        }
    };
}

macro_rules! branch {
    ($s:expr) => {
        $s.db.as_ref().unwrap()
    };
}

impl Connector {
    pub fn new<S: Into<String>>(conn_str: S) -> Self {
        Self {
            conn_str: conn_str.into(),
            db: None,
        }
    }

    pub async fn connect(&mut self) -> Result<&mut Self> {
        if self.db.is_some() {
            return Err(anyhow!("db already connected"));
        }

        let db = match self.conn_str.split_once("://") {
            Some((MYSQL, _)) => FqxPool::M(Pool::<MySql>::connect(&self.conn_str).await?),
            Some((POSTGRES, _)) => FqxPool::P(Pool::<Postgres>::connect(&self.conn_str).await?),
            Some((SQLITE, _)) => FqxPool::S(Pool::<Sqlite>::connect(&self.conn_str).await?),
            _ => {
                return Err(anyhow!(
                    "driver not found, check your connect string: {}",
                    self.conn_str
                ))
            }
        };

        self.db = Some(db);

        Ok(self)
    }

    pub async fn disconnect(&mut self) -> Result<&mut Self> {
        guard!(self);

        match branch!(self) {
            FqxPool::M(p) => p.close().await,
            FqxPool::P(p) => p.close().await,
            FqxPool::S(p) => p.close().await,
        };
        self.db = None;

        Ok(self)
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        guard!(self);

        match branch!(self) {
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
        guard!(self);

        let res = match branch!(self) {
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
        guard!(self);

        let res = match branch!(self) {
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
        guard!(self);

        let res = match branch!(self) {
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
        guard!(self);

        let insert_data = R::insert(data)?;
        let is = match branch!(self) {
            FqxPool::M(_) => insert_data.to_string(MysqlQueryBuilder),
            FqxPool::P(_) => insert_data.to_string(PostgresQueryBuilder),
            FqxPool::S(_) => insert_data.to_string(SqliteQueryBuilder),
        };

        match mode {
            SaveMode::Override => {
                let drop_table = R::drop_table();
                let create_table = R::create_table();
                let (dt, ct) = match branch!(self) {
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
        let mut c = Connector::new(PG_URL);

        c.connect().await.unwrap();
    }
}
