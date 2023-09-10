//! file: sql_sttm_dereive.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx_core::prelude::*;
use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};

#[allow(dead_code)]
#[derive(FqxTable)]
struct MyTable {
    #[fastqx(primary_key, auto_increment)]
    id: i64,
    #[fastqx(unique_key)]
    name: String,
    description: String,
}

#[test]
fn derive_success() {
    let table = MyTable::create_table();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));

    let table = MyTable::drop_table();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));
}
