//! file: sqlx_dyn.rs
//! author: Jacob Xie
//! date: 2023/09/10 22:13:40 Sunday
//! brief:

use std::ops::Deref;

use fastqx_core::prelude::*;
use sqlx::{postgres::PgRow, Column, Row};

#[tokio::test]
async fn fetch_dyn() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let sql = "select * from users";

    let conn = Connector::new(conn_str).unwrap();
    let pool = conn.db().get_p().unwrap();

    let _rows = sqlx::query(sql)
        .try_map(|row: PgRow| {
            // TODO

            let _c = row
                .columns()
                .iter()
                .map(|c| (c.name(), c.type_info().deref()))
                .collect::<Vec<_>>();

            Ok(())
        })
        .fetch(pool);
}
