//! file: pool.rs
//! author: Jacob Xie
//! date: 2023/12/18 23:46:59 Monday
//! brief:

use ref_cast::RefCast;
use sqlx::mysql::MySql;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::sqlite::Sqlite;
use sqlx::Pool;

// ================================================================================================
// Pool
// ================================================================================================

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolMySql(pub(crate) Pool<MySql>);

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolPostgres(pub(crate) Pool<Postgres>);

#[derive(RefCast, Debug, Clone)]
#[repr(transparent)]
pub struct PoolSqlite(pub(crate) Pool<Sqlite>);

// ================================================================================================
// PoolConnection
// ================================================================================================

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionMySql(pub(crate) PoolConnection<MySql>);

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionPostgres(pub(crate) PoolConnection<Postgres>);

#[derive(RefCast, Debug)]
#[repr(transparent)]
pub struct PoolConnectionSqlite(pub(crate) PoolConnection<Sqlite>);

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
