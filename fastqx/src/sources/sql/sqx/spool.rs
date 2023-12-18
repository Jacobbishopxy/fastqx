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
