//! file: dyn.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:23:06 Wednesday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;
use sea_query::Query;
use sea_query::{
    Alias, ColumnDef, InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, SqliteQueryBuilder,
    Table, TableCreateStatement, TableDropStatement,
};

use crate::adt::*;
use crate::sql::*;

// ================================================================================================
// RoughData statements
// ================================================================================================

impl RoughData {
    fn create_table(&self, table_name: &str) -> TableCreateStatement {
        let mut table = Table::create();
        table.table(Alias::new(table_name)).if_not_exists();

        for (col_name, col_type) in self.columns.iter().zip(self.types.iter()) {
            let mut cd = ColumnDef::new(Alias::new(col_name));
            match col_type {
                RoughValueType::Bool => {
                    table.col(cd.boolean());
                }
                RoughValueType::U8 => {
                    table.col(cd.tiny_unsigned());
                }
                RoughValueType::U16 => {
                    table.col(cd.small_unsigned());
                }
                RoughValueType::U32 => {
                    table.col(cd.unsigned());
                }
                RoughValueType::U64 => {
                    table.col(cd.big_unsigned());
                }
                RoughValueType::I8 => {
                    table.col(cd.tiny_integer());
                }
                RoughValueType::I16 => {
                    table.col(cd.small_integer());
                }
                RoughValueType::I32 => {
                    table.col(cd.integer());
                }
                RoughValueType::I64 => {
                    table.col(cd.big_integer());
                }
                RoughValueType::F32 => {
                    table.col(cd.float());
                }
                RoughValueType::F64 => {
                    table.col(cd.double());
                }
                RoughValueType::String => {
                    table.col(cd.string());
                }
                RoughValueType::Blob => {
                    table.col(cd.binary());
                }
                RoughValueType::Null => {
                    table.col(cd.string());
                }
            }
        }

        table.to_owned()
    }

    fn drop_table(&self, table_name: &str) -> TableDropStatement {
        Table::drop().table(Alias::new(table_name)).to_owned()
    }

    fn insert(self, table_name: &str) -> Result<InsertStatement> {
        let mut query = Query::insert();
        let columns = self
            .columns
            .iter()
            .map(|c| Alias::new(c))
            .collect::<Vec<_>>();
        query.into_table(Alias::new(table_name)).columns(columns);

        for row in self.data.into_iter() {
            query.values(
                row.into_iter()
                    .map(|e| match e {
                        RoughValue::Bool(v) => v.into(),
                        RoughValue::U8(v) => v.into(),
                        RoughValue::U16(v) => v.into(),
                        RoughValue::U32(v) => v.into(),
                        RoughValue::U64(v) => v.into(),
                        RoughValue::I8(v) => v.into(),
                        RoughValue::I16(v) => v.into(),
                        RoughValue::I32(v) => v.into(),
                        RoughValue::I64(v) => v.into(),
                        RoughValue::F32(v) => v.into(),
                        RoughValue::F64(v) => v.into(),
                        RoughValue::String(v) => v.into(),
                        RoughValue::Blob(v) => v.into(),
                        RoughValue::Null => Option::<String>::None.into(), // Option type doesn't effect 'Null' value
                    })
                    .collect::<Vec<_>>(),
            )?;
        }

        Ok(query.to_owned())
    }
}

// ================================================================================================
// Connector dyn fn
// ================================================================================================

impl Connector {
    pub async fn dyn_fetch(&self, sql: &str) -> Result<RoughData> {
        let mut proc = SqlxRowProcessor::new();

        let stream = match self.db() {
            FqxPool::M(p) => sqlx::query(sql).try_map(|r| proc.process(r)).fetch(p),
            FqxPool::P(p) => sqlx::query(sql).try_map(|r| proc.process(r)).fetch(p),
            FqxPool::S(p) => sqlx::query(sql).try_map(|r| proc.process(r)).fetch(p),
        };

        let data = stream.try_collect::<Vec<_>>().await?;

        Ok(RoughData {
            columns: proc.columns().unwrap(),
            types: proc.types().unwrap(),
            data,
        })
    }

    pub async fn dyn_save(
        &self,
        mut data: RoughData,
        table_name: &str,
        mode: SaveMode,
    ) -> Result<()> {
        // make sure each row has the same type series
        data.type_coercion()?;

        match mode {
            SaveMode::Override => {
                let drop_table = data.drop_table(table_name);
                let create_table = data.create_table(table_name);
                let (dt, ct) = match self.db() {
                    FqxPool::M(_) => (
                        drop_table.to_string(MysqlQueryBuilder),
                        create_table.to_string(MysqlQueryBuilder),
                    ),
                    FqxPool::P(_) => (
                        drop_table.to_string(PostgresQueryBuilder),
                        create_table.to_string(PostgresQueryBuilder),
                    ),
                    FqxPool::S(_) => (
                        drop_table.to_string(SqliteQueryBuilder),
                        create_table.to_string(SqliteQueryBuilder),
                    ),
                };
                let is = _dyn_insert_data(self.db(), data, table_name)?;

                let _ = self.execute(&dt).await;
                self.execute(&ct).await?;
                self.execute(&is).await?;
            }
            SaveMode::Append => {
                let is = _dyn_insert_data(self.db(), data, table_name)?;
                self.execute(&is).await?;
            }
        }

        Ok(())
    }
}

fn _dyn_insert_data(db: &FqxPool, data: RoughData, table_name: &str) -> Result<String> {
    let insert_data = data.insert(table_name)?;
    let res = match db {
        FqxPool::M(_) => insert_data.to_string(MysqlQueryBuilder),
        FqxPool::P(_) => insert_data.to_string(PostgresQueryBuilder),
        FqxPool::S(_) => insert_data.to_string(SqliteQueryBuilder),
    };

    Ok(res)
}
