//! file: db.rs
//! author: Jacob Xie
//! date: 2023/09/09 18:51:43 Saturday
//! brief:

use anyhow::{anyhow, Result};
use sqlx::database::HasArguments;
use sqlx::mysql::MySql;
use sqlx::postgres::Postgres;
use sqlx::sqlite::Sqlite;
use sqlx::{Database, Executor, FromRow, IntoArguments, Pool};

// ================================================================================================
// Const
// ================================================================================================

const MYSQL: &str = "mysql";
const POSTGRES: &str = "postgres";
const SQLITE: &str = "sqlite";

// ================================================================================================
// Connector<D>
// ================================================================================================

#[derive(Debug, Clone)]
pub struct Connector<D>
where
    D: Database,
    for<'e> &'e Pool<D>: Executor<'e, Database = D>,
    for<'e> <D as HasArguments<'e>>::Arguments: IntoArguments<'e, D>,
{
    conn_str: String,
    db: Option<Pool<D>>,
}

impl<D> Connector<D>
where
    D: Database,
    for<'e> &'e Pool<D>: Executor<'e, Database = D>,
    for<'e> <D as HasArguments<'e>>::Arguments: IntoArguments<'e, D>,
{
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

        let db = Pool::<D>::connect(&self.conn_str).await?;
        self.db = Some(db);

        Ok(self)
    }

    pub async fn disconnect(&mut self) -> Result<&mut Self> {
        if self.db.is_none() {
            return Err(anyhow!("db not connect"));
        }

        self.db.as_ref().unwrap().close().await;
        self.db = None;

        Ok(self)
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        if self.db.is_none() {
            return Err(anyhow!("db not connect"));
        }

        sqlx::query(sql).execute(self.db.as_ref().unwrap()).await?;

        Ok(())
    }

    pub async fn fetch_all<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: for<'r> FromRow<'r, D::Row>,
        R: Send + Unpin,
    {
        if self.db.is_none() {
            return Err(anyhow!("db not connect"));
        }

        let p = self.db.as_ref().unwrap();
        Ok(sqlx::query_as::<_, R>(sql).fetch_all(p).await?)
    }

    pub async fn fetch_one<R>(&self, sql: &str) -> Result<R>
    where
        R: for<'r> FromRow<'r, D::Row>,
        R: Send + Unpin,
    {
        if self.db.is_none() {
            return Err(anyhow!("db not connect"));
        }

        let p = self.db.as_ref().unwrap();
        Ok(sqlx::query_as::<_, R>(sql).fetch_one(p).await?)
    }

    pub async fn fetch_optional<R>(&self, sql: &str) -> Result<Option<R>>
    where
        R: for<'r> FromRow<'r, D::Row>,
        R: Send + Unpin,
    {
        if self.db.is_none() {
            return Err(anyhow!("db not connect"));
        }

        let p = self.db.as_ref().unwrap();
        Ok(sqlx::query_as::<_, R>(sql).fetch_optional(p).await?)
    }

    pub async fn save<R>(&self, _table: &str, _data: Vec<R>, _mode: SaveMode)
    where
        R: for<'r> FromRow<'r, D::Row>,
        R: Send + Unpin,
    {
        todo!()
    }
}

pub enum SaveMode {
    Override,
    Append,
    CreateIfNotExist,
}

// ================================================================================================
// FastqxDB
// ================================================================================================

pub enum FastqxDB {
    M(Connector<MySql>),
    P(Connector<Postgres>),
    S(Connector<Sqlite>),
}

impl FastqxDB {
    pub fn new(conn_str: &str) -> Result<Self> {
        match conn_str.split_once("://") {
            Some((MYSQL, _)) => Ok(Self::M(Connector::<MySql>::new(conn_str))),
            Some((POSTGRES, _)) => Ok(Self::P(Connector::<Postgres>::new(conn_str))),
            Some((SQLITE, _)) => Ok(Self::S(Connector::<Sqlite>::new(conn_str))),
            _ => Err(anyhow!(
                "driver not found, check your connect string: {}",
                conn_str
            )),
        }
    }
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
        let mut c = Connector::<Postgres>::new(PG_URL);

        c.connect().await.unwrap();
    }
}
