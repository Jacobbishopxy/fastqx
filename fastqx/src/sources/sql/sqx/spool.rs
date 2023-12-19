//! file: pool.rs
//! author: Jacob Xie
//! date: 2023/12/18 23:46:59 Monday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;
use ref_cast::RefCast;
use sqlx::mysql::{MySql, MySqlRow};
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::sqlite::{Sqlite, SqliteRow};
use sqlx::Pool;

use super::FromSqlxRow;

// ================================================================================================
// Pool
// ================================================================================================

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolMySql(Pool<MySql>);

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolPostgres(Pool<Postgres>);

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolSqlite(Pool<Sqlite>);

// ================================================================================================
// PoolConnection
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionMySql(PoolConnection<MySql>);

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionPostgres(PoolConnection<Postgres>);

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionSqlite(PoolConnection<Sqlite>);

// ================================================================================================
// AsRef
// ================================================================================================

impl AsRef<Pool<MySql>> for PoolMySql {
    fn as_ref(&self) -> &Pool<MySql> {
        &self.0
    }
}

impl AsRef<Pool<Postgres>> for PoolPostgres {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl AsRef<Pool<Sqlite>> for PoolSqlite {
    fn as_ref(&self) -> &Pool<Sqlite> {
        &self.0
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl PoolMySql {
    pub async fn new(
        host: &str,
        port: Option<u16>,
        user: &str,
        pass: &str,
        db: &str,
    ) -> Result<Self> {
        let cs = &gen_sqlx_str("mysql", host, port.unwrap_or(3306), user, pass, db);
        Ok(Self(Pool::<MySql>::connect(cs).await?))
    }

    pub async fn new_from_str(url: &str) -> Result<Self> {
        Ok(Self(Pool::<MySql>::connect(url).await?))
    }

    pub async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    pub async fn acquire(&self) -> Result<PoolConnectionMySql> {
        Ok(PoolConnectionMySql(self.0.acquire().await?))
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FromSqlxRow<MySqlRow>,
    {
        let stream = sqlx::query(sql).try_map(R::from_row).fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

impl PoolPostgres {
    pub async fn new(
        host: &str,
        port: Option<u16>,
        user: &str,
        pass: &str,
        db: &str,
    ) -> Result<Self> {
        let cs = &gen_sqlx_str("postgres", host, port.unwrap_or(5432), user, pass, db);
        Ok(Self(Pool::<Postgres>::connect(cs).await?))
    }

    pub async fn new_from_str(url: &str) -> Result<Self> {
        Ok(Self(Pool::<Postgres>::connect(url).await?))
    }

    pub async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    pub async fn acquire(&self) -> Result<PoolConnectionPostgres> {
        Ok(PoolConnectionPostgres(self.0.acquire().await?))
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FromSqlxRow<PgRow>,
    {
        let stream = sqlx::query(sql).try_map(R::from_row).fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

impl PoolSqlite {
    pub async fn new_from_str(url: &str) -> Result<Self> {
        Ok(Self(Pool::<Sqlite>::connect(url).await?))
    }

    pub async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    pub async fn acquire(&self) -> Result<PoolConnectionSqlite> {
        Ok(PoolConnectionSqlite(self.0.acquire().await?))
    }

    pub async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FromSqlxRow<SqliteRow>,
    {
        let stream = sqlx::query(sql).try_map(R::from_row).fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

// ================================================================================================
// helpers
// ================================================================================================

fn gen_sqlx_str(driver: &str, host: &str, port: u16, user: &str, pass: &str, db: &str) -> String {
    format!("{driver}://{user}:{pass}@{host}:{port}/{db}")
}
