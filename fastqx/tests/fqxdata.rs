//! file: rough_data.rs
//! author: Jacob Xie
//! date: 2023/09/13 22:01:42 Wednesday
//! brief:

use fastqx::prelude::*;
use once_cell::sync::Lazy;

static CONN_PG: &str = "postgres://dev:devpass@localhost:5437/dev";
static CONN_MS: &str =
    "jdbc:sqlserver://localhost:1433;username=dev;password=StrongPassword123;databaseName=devdb;encrypt=true;integratedSecurity=true;";

static DATA: Lazy<FqxData> = Lazy::new(|| {
    FqxData::new(
        vec![String::from("c1"), String::from("c2"), String::from("c3")],
        vec![FqxValueType::I32, FqxValueType::String, FqxValueType::F32],
        vec![
            vec![
                FqxValue::I32(1),
                FqxValue::String(String::from("A")),
                FqxValue::F32(2.1),
            ],
            vec![
                FqxValue::I32(2),
                FqxValue::String(String::from("B")),
                FqxValue::F32(1.3),
            ],
            vec![
                FqxValue::I32(3),
                FqxValue::String(String::from("C")),
                FqxValue::F32(3.2),
            ],
        ],
    )
    .unwrap()
});

#[tokio::test]
async fn dyn_save_pg_success() {
    let conn = Connector::new(CONN_PG).await.unwrap();

    let res = conn
        .dyn_save(DATA.clone(), "tmp_table", SaveMode::Override, false)
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_pg_success() {
    let conn = Connector::new(CONN_PG).await.unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn dyn_save_ms_success() {
    let conn = Connector::new(CONN_MS).await.unwrap();

    let res = conn
        .dyn_save(DATA.clone(), "tmp_table", SaveMode::Override, false)
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_ms_success() {
    let conn = Connector::new(CONN_MS).await.unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}
