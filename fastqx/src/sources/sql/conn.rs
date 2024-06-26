//! file: conn.rs
//! author: Jacob Xie
//! date: 2023/09/09 18:51:43 Saturday
//! brief:

use anyhow::{bail, Result};
use pyo3::pyclass;

use super::ab::{gen_sqlx_str, FqxSqlPool, FqxSqlRow};
use super::adt::*;
use super::sqx::*;
use super::tbr::*;
use crate::sources::SaveMode;

// ================================================================================================
// FqxPool & FqxPoolConnection
// ================================================================================================

#[derive(Debug, Clone)]
pub enum FqxPool {
    M(PoolMySql),
    P(PoolPostgres),
    S(PoolSqlite),
    Q(PoolMsSql),
}

impl FqxPool {
    pub fn get_m(&self) -> Option<&PoolMySql> {
        match self {
            FqxPool::M(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_p(&self) -> Option<&PoolPostgres> {
        match self {
            FqxPool::P(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_s(&self) -> Option<&PoolSqlite> {
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
    M(PoolConnectionMySql),
    P(PoolConnectionPostgres),
    S(PoolConnectionSqlite),
    Q(PoolConnectionMsSql),
}

impl FqxPoolConnection {
    pub fn get_m(&self) -> Option<&PoolConnectionMySql> {
        match self {
            FqxPoolConnection::M(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_p(&self) -> Option<&PoolConnectionPostgres> {
        match self {
            FqxPoolConnection::P(p) => Some(p),
            _ => None,
        }
    }

    pub fn get_s(&self) -> Option<&PoolConnectionSqlite> {
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
// SqlConnector
// ================================================================================================

#[pyclass]
#[derive(Debug, Clone)]
pub struct SqlConnector {
    driver: Driver,
    conn_str: String,
    db: FqxPool,
}

impl SqlConnector {
    pub async fn new(
        driver: Driver,
        host: &str,
        port: Option<u16>,
        user: &str,
        pass: &str,
        db: &str,
    ) -> Result<Self> {
        let (p, driver_str, driver, pt) = match driver {
            Driver::MYSQL => (
                FqxPool::M(PoolMySql::new(host, port, user, pass, db).await?),
                "mysql",
                Driver::MYSQL,
                port.unwrap_or(3306),
            ),
            Driver::POSTGRES => (
                FqxPool::P(PoolPostgres::new(host, port, user, pass, db).await?),
                "postgres",
                Driver::POSTGRES,
                port.unwrap_or(5432),
            ),
            Driver::MSSQL => (
                FqxPool::Q(PoolMsSql::new(host, port, user, pass, db).await?),
                "mssql",
                Driver::MSSQL,
                port.unwrap_or(1433),
            ),
            Driver::SQLITE => panic!("PoolSqlite does not have `new` method!"),
        };

        let conn_str = gen_sqlx_str(driver_str, host, pt, user, pass, db);

        Ok(Self {
            driver,
            conn_str,
            db: p,
        })
    }

    pub async fn new_by_str<S: Into<String>>(conn_str: S) -> Result<Self> {
        let conn_str = conn_str.into();
        let (db, driver) = match &conn_str.split_once("://") {
            Some((MYSQL, _)) => (
                FqxPool::M(PoolMySql::new_by_str(&conn_str).await?),
                Driver::MYSQL,
            ),
            Some((POSTGRES, _)) => (
                FqxPool::P(PoolPostgres::new_by_str(&conn_str).await?),
                Driver::POSTGRES,
            ),
            Some((SQLITE, _)) => (
                FqxPool::S(PoolSqlite::new_by_str(&conn_str).await?),
                Driver::SQLITE,
            ),
            Some((MSSQL, _)) => (
                FqxPool::Q(PoolMsSql::new_by_str(&conn_str).await?),
                Driver::MSSQL,
            ),
            _ => {
                bail!("driver not found, check your connect string: {}", &conn_str)
            }
        };

        Ok(Self {
            driver,
            conn_str: conn_str.into(),
            db,
        })
    }

    pub fn driver(&self) -> &Driver {
        &self.driver
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
            FqxPool::M(p) => p.close().await?,
            FqxPool::P(p) => p.close().await?,
            FqxPool::S(p) => p.close().await?,
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
                p.execute(sql).await?;
            }
            FqxPool::P(p) => {
                p.execute(sql).await?;
            }
            FqxPool::S(p) => {
                p.execute(sql).await?;
            }
            FqxPool::Q(p) => {
                p.execute(sql).await?;
            }
        };

        Ok(())
    }

    pub async fn fetch<R>(&self, sql: &str) -> Result<Vec<R>>
    where
        R: FqxSqlRow,
    {
        let res = match self.db() {
            FqxPool::M(p) => p.fetch::<R>(sql).await?,
            FqxPool::P(p) => p.fetch::<R>(sql).await?,
            FqxPool::S(p) => p.fetch::<R>(sql).await?,
            FqxPool::Q(p) => p.fetch::<R>(sql).await?,
        };

        Ok(res)
    }

    pub async fn save<R>(&self, data: Vec<R>, mode: SaveMode) -> Result<()>
    where
        R: FqxSqlRow,
    {
        match &self.db {
            FqxPool::M(p) => p.save(data, mode).await,
            FqxPool::P(p) => p.save(data, mode).await,
            FqxPool::S(p) => p.save(data, mode).await,
            FqxPool::Q(p) => p.save(data, mode).await,
        }
    }
}

// ================================================================================================
// Test
// ================================================================================================

#[cfg(test)]
mod test_db {
    use crate::sources::sql::rowprocess::FqxSqlRowProcessor;

    use super::*;

    use futures::TryStreamExt;

    const PG_URL: &str = "postgresql://dev:devpass@localhost:5437/dev";

    #[tokio::test]
    async fn test_conn() {
        let c = SqlConnector::new_by_str(PG_URL).await.unwrap();

        c.acquire().await.unwrap();
    }

    #[tokio::test]
    async fn fetch_dyn2() {
        let conn = SqlConnector::new_by_str(PG_URL).await.unwrap();

        let sql = "select * from users";
        let pool = conn.db().get_p().unwrap();

        let mut proc = FqxSqlRowProcessor::new();

        let stream = sqlx::query(sql)
            .try_map(|r| proc.process_sqlx_row(r))
            .fetch(pool.as_ref());

        let res = stream.try_collect::<Vec<_>>().await.unwrap();

        println!("{:?}", res);
    }
}
