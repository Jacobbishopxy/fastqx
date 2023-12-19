//! file: pool.rs
//! author: Jacob Xie
//! date: 2023/12/19 20:40:50 Tuesday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;
use sqlx::mysql::{MySql, MySqlRow};
use sqlx::postgres::{PgRow, Postgres};
use sqlx::sqlite::{Sqlite, SqliteRow};

use super::adt::*;
use super::sqx::*;
use super::tbr::*;
use crate::sources::SaveMode;

// ================================================================================================
// FqxSqlRow
//
// A struct who derived `FqxSchema` auto impl this trait, see `tests/sql_sttm.rs`
// ================================================================================================

pub trait FqxSqlRow
where
    Self: Send + Unpin,
    Self: FromSqlxRow<MySqlRow>,
    Self: FromSqlxRow<PgRow>,
    Self: FromSqlxRow<SqliteRow>,
    Self: FromTiberiusRow,
{
    fn create_table(driver: &Driver) -> Result<String>;

    fn drop_table(driver: &Driver) -> Result<String>;

    fn insert<I: IntoIterator<Item = Self>>(driver: &Driver, data: I) -> Result<String>;
}

// ================================================================================================
// FqxSqlPool
// ================================================================================================

pub(crate) trait FqxSqlPool: Sized {
    type PoolConnection;

    fn driver() -> Driver;

    async fn new(host: &str, port: Option<u16>, user: &str, pass: &str, db: &str) -> Result<Self>;

    async fn new_by_str(url: &str) -> Result<Self>;

    async fn close(&self) -> Result<()>;

    fn is_closed(&self) -> bool;

    async fn acquire(&self) -> Result<Self::PoolConnection>;

    async fn execute(&self, sql: &str) -> Result<()>;

    async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow;

    async fn save<I, R>(&self, data: I, mode: SaveMode) -> Result<()>
    where
        I: IntoIterator<Item = R>,
        R: FqxSqlRow,
    {
        let insert_data = R::insert(&Self::driver(), data)?;

        match mode {
            SaveMode::Override => {
                let drop_table = R::drop_table(&Self::driver())?;
                let create_table = R::create_table(&Self::driver())?;

                let _ = self.execute(&drop_table).await;
                self.execute(&create_table).await?;
                self.execute(&insert_data).await?;
            }
            SaveMode::Append => {
                self.execute(&insert_data).await?;
            }
        }

        Ok(())
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl FqxSqlPool for PoolMsSql {
    type PoolConnection = PoolConnectionMsSql;

    fn driver() -> Driver {
        Driver::MSSQL
    }

    async fn new(host: &str, port: Option<u16>, user: &str, pass: &str, db: &str) -> Result<Self> {
        let m = MsSqlConnectionManager::new(host, port, user, pass, db)?;
        let pool = bb8::Pool::builder().build(m).await?;

        Ok(Self(pool))
    }

    async fn new_by_str(url: &str) -> Result<Self> {
        let m = MsSqlConnectionManager::new_from_str(url)?;

        let pool = bb8::Pool::builder().build(m).await?;

        Ok(Self(pool))
    }

    async fn close(&self) -> Result<()> {
        // TODO: `close` method takes ownership
        // let conn = self.0.get_owned().await?;
        // conn.close().await?;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        false
    }

    async fn acquire(&self) -> Result<Self::PoolConnection> {
        Ok(self.0.get_owned().await?)
    }

    async fn execute(&self, sql: &str) -> Result<()> {
        let mut conn = self.0.get().await?;

        conn.execute(sql, &[]).await?;

        Ok(())
    }

    async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow,
    {
        let mut conn = self.0.get().await?;

        let query = conn.simple_query(sql).await?;

        let mut stream = query.into_row_stream();
        let mut res = vec![];

        while let Ok(Some(row)) = stream.try_next().await {
            let r = <R as FromTiberiusRow>::from_row(row)?;
            res.push(r);
        }

        Ok(res)
    }
}

impl FqxSqlPool for PoolMySql {
    type PoolConnection = PoolConnectionMySql;

    fn driver() -> Driver {
        Driver::MYSQL
    }

    async fn new(host: &str, port: Option<u16>, user: &str, pass: &str, db: &str) -> Result<Self> {
        let cs = &gen_sqlx_str("mysql", host, port.unwrap_or(3306), user, pass, db);
        Ok(Self(sqlx::Pool::<MySql>::connect(cs).await?))
    }

    async fn new_by_str(url: &str) -> Result<Self> {
        Ok(Self(sqlx::Pool::<MySql>::connect(url).await?))
    }

    async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    async fn acquire(&self) -> Result<Self::PoolConnection> {
        Ok(PoolConnectionMySql(self.0.acquire().await?))
    }

    async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow,
    {
        let stream = sqlx::query(sql)
            .try_map(<R as FromSqlxRow<MySqlRow>>::from_row)
            .fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

impl FqxSqlPool for PoolPostgres {
    type PoolConnection = PoolConnectionPostgres;

    fn driver() -> Driver {
        Driver::POSTGRES
    }

    async fn new(host: &str, port: Option<u16>, user: &str, pass: &str, db: &str) -> Result<Self> {
        let cs = &gen_sqlx_str("postgres", host, port.unwrap_or(5432), user, pass, db);
        Ok(Self(sqlx::Pool::<Postgres>::connect(cs).await?))
    }

    async fn new_by_str(url: &str) -> Result<Self> {
        Ok(Self(sqlx::Pool::<Postgres>::connect(url).await?))
    }

    async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    async fn acquire(&self) -> Result<Self::PoolConnection> {
        Ok(PoolConnectionPostgres(self.0.acquire().await?))
    }

    async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow,
    {
        let stream = sqlx::query(sql)
            .try_map(<R as FromSqlxRow<PgRow>>::from_row)
            .fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

impl FqxSqlPool for PoolSqlite {
    type PoolConnection = PoolConnectionSqlite;

    fn driver() -> Driver {
        Driver::SQLITE
    }

    async fn new(
        _host: &str,
        _port: Option<u16>,
        _user: &str,
        _pass: &str,
        _db: &str,
    ) -> Result<Self> {
        panic!("PoolSqlite does not have `new` method!")
    }

    async fn new_by_str(url: &str) -> Result<Self> {
        Ok(Self(sqlx::Pool::<Sqlite>::connect(url).await?))
    }

    async fn close(&self) -> Result<()> {
        self.0.close().await;
        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    async fn acquire(&self) -> Result<Self::PoolConnection> {
        Ok(PoolConnectionSqlite(self.0.acquire().await?))
    }

    async fn execute(&self, sql: &str) -> Result<()> {
        sqlx::query(sql).execute(&self.0).await?;
        Ok(())
    }

    async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow,
    {
        let stream = sqlx::query(sql)
            .try_map(<R as FromSqlxRow<SqliteRow>>::from_row)
            .fetch(&self.0);
        Ok(stream.try_collect::<Vec<_>>().await?)
    }
}

// ================================================================================================
// helper
// ================================================================================================

fn gen_sqlx_str(driver: &str, host: &str, port: u16, user: &str, pass: &str, db: &str) -> String {
    format!("{driver}://{user}:{pass}@{host}:{port}/{db}")
}
