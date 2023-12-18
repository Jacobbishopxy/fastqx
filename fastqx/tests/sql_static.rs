//! file: sql_static.rs
//! author: Jacob Xie
//! date: 2023/09/10 09:34:02 Sunday
//! brief:

use fastqx::prelude::*;
use once_cell::sync::Lazy;

#[derive(Clone, FqxSql, Debug)]
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
    let driver = Driver::POSTGRES;

    let create_table = Users::create_table(&driver);
    println!("{:?}", create_table);

    let drop_table = Users::drop_table(&driver);
    println!("{:?}", drop_table);

    let insert = Users::insert(&driver, DATA.clone());
    println!("{:?}", insert);
}

#[tokio::test]
async fn to_postgres_success() {
    let conn_str = "postgres://dev:devpass@localhost:5437/dev";

    let conn = SqlConnector::new(conn_str).await.unwrap();

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
    let conn_str = "mssql://dev:StrongPassword123@localhost:1433/devdb";

    let conn = SqlConnector::new(conn_str).await.unwrap();

    conn.save(DATA.clone(), SaveMode::Override).await.unwrap();

    let res = conn.fetch::<Users>("select * from users").await.unwrap();

    println!("{:?}", res);
}
