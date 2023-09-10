//! file: sql_sttm_dereive.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx_core::conn::db::{Connector, SaveMode};
use fastqx_core::prelude::*;
use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};

#[derive(FqxSchema)]
struct MyTable {
    #[fastqx(primary_key, auto_increment)]
    id: i64,
    #[fastqx(unique_key)]
    name: String,
    description: Option<String>,
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

    let data = vec![
        MyTable {
            id: 1,
            name: String::from("Jacob"),
            description: None,
        },
        MyTable {
            id: 2,
            name: String::from("Mia"),
            description: Some(String::from("K")),
        },
        MyTable {
            id: 3,
            name: String::from("White"),
            description: Some(String::from("J.W")),
        },
    ];

    let insert = MyTable::insert(data).unwrap();

    println!("{:?}", insert.to_string(MysqlQueryBuilder));
    println!("{:?}", insert.to_string(PostgresQueryBuilder));
    println!("{:?}", insert.to_string(SqliteQueryBuilder));
}

#[tokio::test]
async fn to_postgres_success() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let mut conn = Connector::new(conn_str);
    conn.connect().await.unwrap();

    // 1. create table

    let create_table = MyTable::create_table().to_string(PostgresQueryBuilder);

    conn.execute(&create_table).await.unwrap();

    // 2. insert data

    let data = vec![
        MyTable {
            id: 1,
            name: String::from("Jacob"),
            description: None,
        },
        MyTable {
            id: 2,
            name: String::from("Mia"),
            description: Some(String::from("K")),
        },
        MyTable {
            id: 3,
            name: String::from("White"),
            description: Some(String::from("J.W")),
        },
    ];

    conn.save(data, SaveMode::Append).await.unwrap();
}
