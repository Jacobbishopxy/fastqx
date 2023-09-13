//! file: rough_data.rs
//! author: Jacob Xie
//! date: 2023/09/13 22:01:42 Wednesday
//! brief:

use fastqx_core::prelude::*;

static CONN_STR: &str = "postgres://dev:devpass@localhost:5437/dev";

#[tokio::test]
async fn dyn_save_success() {
    let data = RoughData::new(
        vec![String::from("c1"), String::from("c2"), String::from("c3")],
        vec![
            RoughValueType::I32,
            RoughValueType::String,
            RoughValueType::F32,
        ],
        vec![
            vec![
                RoughValue::I32(1),
                RoughValue::String(String::from("A")),
                RoughValue::F32(2.1),
            ],
            vec![
                RoughValue::I32(2),
                RoughValue::String(String::from("B")),
                RoughValue::F32(1.3),
            ],
            vec![
                RoughValue::I32(3),
                RoughValue::String(String::from("C")),
                RoughValue::F32(3.2),
            ],
        ],
    )
    .unwrap();

    let conn = Connector::new(CONN_STR).unwrap();

    let res = conn.dyn_save(data, "tmp_table", SaveMode::Override).await;
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn dyn_fetch_success() {
    let conn = Connector::new(CONN_STR).unwrap();

    let data = conn.dyn_fetch("select * from tmp_table").await;
    println!("{:?}", data);
    assert!(data.is_ok());
}
