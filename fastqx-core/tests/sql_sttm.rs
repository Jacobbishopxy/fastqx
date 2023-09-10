//! file: sql_sttm.rs
//! author: Jacob Xie
//! date: 2023/09/10 00:03:25 Sunday
//! brief: https://docs.rs/sea-query/latest/sea_query/index.html

use sea_query::{
    Alias, ColumnDef, ColumnType, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder,
    Table, Value,
};

#[test]
fn create_table() {
    let table = Table::create()
        .table(Alias::new("tmp_table"))
        .if_not_exists()
        .col(
            ColumnDef::new(Alias::new("id"))
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Alias::new("font_size")).integer().not_null())
        .col(ColumnDef::new(Alias::new("charactor")).string().not_null())
        .col(ColumnDef::new_with_type(Alias::new("size_w"), ColumnType::Integer).not_null())
        .col(ColumnDef::new(Alias::new("size_h")).integer().not_null())
        .col(
            ColumnDef::new(Alias::new("font_id"))
                .integer()
                .default(Value::Int(None)),
        )
        .to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));
}

#[test]
fn drop_table() {
    let table = Table::drop().table(Alias::new("tmp_table")).to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
    println!("{:?}", table.to_string(SqliteQueryBuilder));
}

#[test]
fn truncate_table() {
    let table = Table::truncate().table(Alias::new("tmp_table")).to_owned();

    println!("{:?}", table.to_string(MysqlQueryBuilder));
    println!("{:?}", table.to_string(PostgresQueryBuilder));
}
