//! file: sql_sttm_derive.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx::prelude::*;
use once_cell::sync::Lazy;
use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};

#[derive(Clone, FqxSchema, Debug)]
struct Users {
    #[fastqx(primary_key, auto_increment)]
    id: i64,
    #[fastqx(unique_key)]
    name: String,
    description: Option<String>,
    score: f32,
}

static DATA: Lazy<Vec<Users>> = Lazy::new(|| {
    vec![
        Users {
            id: 1,
            name: String::from("Jacob"),
            description: None,
            score: 5.0,
        },
        Users {
            id: 2,
            name: String::from("Mia"),
            description: Some(String::from("K")),
            score: 4.5,
        },
        Users {
            id: 3,
            name: String::from("White"),
            description: Some(String::from("J.W")),
            score: 4.7,
        },
    ]
});

#[test]
fn derive_success() {
    let table = Users::create_table();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));

    let table = Users::drop_table();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));

    let insert = Users::insert(DATA.clone()).unwrap();

    println!("{:?}", insert.to_string(MysqlQueryBuilder));
    println!("{:?}", insert.to_string(PostgresQueryBuilder));
    println!("{:?}", insert.to_string(SqliteQueryBuilder));
}

#[tokio::test]
async fn to_postgres_success() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let conn = Connector::new(conn_str).await.unwrap();

    // 1. create table

    // let create_table = Users::create_table().to_string(PostgresQueryBuilder);
    // conn.execute(&create_table).await.unwrap();

    // 2. insert data

    conn.save(DATA.clone(), SaveMode::Override).await.unwrap();

    // 3. query data

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}

#[tokio::test]
async fn to_mssql_success() {
    let conn_str =
        "jdbc:sqlserver://localhost:1433;username=sa;password=Dev_123a;databaseName=master";

    let conn = Connector::new(conn_str).await.unwrap();

    conn.save(DATA.clone(), SaveMode::Override).await.unwrap();

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}
