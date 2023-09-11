//! file: sqlx_dyn.rs
//! author: Jacob Xie
//! date: 2023/09/10 22:13:40 Sunday
//! brief:

use fastqx_core::prelude::*;
use futures::TryStreamExt;
use sqlx::{postgres::PgRow, Column, Row, TypeInfo};

static CONN_STR: &str = "postgres://dev:devpass@localhost:5437/dev";

#[tokio::test]
async fn fetch_dyn() {
    let conn = Connector::new(CONN_STR).unwrap();

    let sql = "select * from users";
    let pool = conn.db().get_p().unwrap();

    let _rows = sqlx::query(sql)
        .try_map(|row: PgRow| {
            let _c = row
                .columns()
                .iter()
                .map(|c| (c.name(), c.type_info().name()))
                .collect::<Vec<_>>();

            Ok(())
        })
        .fetch(pool);
}

#[tokio::test]
async fn fetch_dyn2() {
    let conn = Connector::new(CONN_STR).unwrap();

    let sql = "select * from users";
    let pool = conn.db().get_p().unwrap();

    let mut proc = SqlxRowProcessor::new();

    let stream = sqlx::query(sql).try_map(|r| proc.process(r)).fetch(pool);

    let res = stream.try_collect::<Vec<_>>().await.unwrap();

    println!("{:?}", res);
}
