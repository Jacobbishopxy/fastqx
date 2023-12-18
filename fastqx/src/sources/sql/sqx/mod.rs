//! file: mod.rs
//! author: Jacob Xie
//! date: 2023/09/19 00:36:57 Tuesday
//! brief:

pub mod srow;

use sqlx::mysql::MySql;
use sqlx::pool::PoolConnection;
use sqlx::postgres::Postgres;
use sqlx::sqlite::Sqlite;
use sqlx::Pool;

pub type PoolMySql = Pool<MySql>;
pub type PoolPostgres = Pool<Postgres>;
pub type PoolSqlite = Pool<Sqlite>;

pub type PoolConnectionMySql = PoolConnection<MySql>;
pub type PoolConnectionPostgres = PoolConnection<Postgres>;
pub type PoolConnectionSqlite = PoolConnection<Sqlite>;
