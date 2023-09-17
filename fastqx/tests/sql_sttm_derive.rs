//! file: sql_sttm_derive.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx::prelude::*;
use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder};

#[derive(FqxSchema, Debug)]
struct Users {
    #[fastqx(primary_key, auto_increment)]
    id: i64,
    #[fastqx(unique_key)]
    name: String,
    description: Option<String>,
}

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

    let data = vec![
        Users {
            id: 1,
            name: String::from("Jacob"),
            description: None,
        },
        Users {
            id: 2,
            name: String::from("Mia"),
            description: Some(String::from("K")),
        },
        Users {
            id: 3,
            name: String::from("White"),
            description: Some(String::from("J.W")),
        },
    ];

    let insert = Users::insert(data).unwrap();

    println!("{:?}", insert.to_string(MysqlQueryBuilder));
    println!("{:?}", insert.to_string(PostgresQueryBuilder));
    println!("{:?}", insert.to_string(SqliteQueryBuilder));
}

#[tokio::test]
async fn to_postgres_success() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let conn = Connector::new(conn_str).unwrap();

    // 1. create table

    // let create_table = Users::create_table().to_string(PostgresQueryBuilder);
    // conn.execute(&create_table).await.unwrap();

    // 2. insert data

    let data = vec![
        Users {
            id: 1,
            name: String::from("Jacob"),
            description: None,
        },
        Users {
            id: 2,
            name: String::from("Mia"),
            description: Some(String::from("K")),
        },
        Users {
            id: 3,
            name: String::from("White"),
            description: Some(String::from("J.W")),
        },
    ];

    conn.save(data, SaveMode::Override).await.unwrap();

    // 3. query data

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}
