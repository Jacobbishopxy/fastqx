//! file: dyn.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:23:06 Wednesday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;
use sea_query::*;

use super::rowprocess::FqxSqlRowProcessor;
use crate::adt::*;
use crate::sources::sql::tbr::sqlbuild as mssql_sqlbuild;
use crate::sources::sql::*;

// ================================================================================================
// FqxData statements
// ================================================================================================

impl FqxData {
    fn create_sqlx_table(&self, table_name: &str) -> TableCreateStatement {
        let mut table = Table::create();
        table.table(Alias::new(table_name)).if_not_exists();

        for (col_name, col_type) in self.columns.iter().zip(self.types.iter()) {
            let mut cd = ColumnDef::new(Alias::new(col_name));
            match col_type {
                FqxValueType::Bool => {
                    table.col(cd.boolean());
                }
                FqxValueType::U8 => {
                    table.col(cd.tiny_unsigned());
                }
                FqxValueType::U16 => {
                    table.col(cd.small_unsigned());
                }
                FqxValueType::U32 => {
                    table.col(cd.unsigned());
                }
                FqxValueType::U64 => {
                    table.col(cd.big_unsigned());
                }
                FqxValueType::I8 => {
                    table.col(cd.tiny_integer());
                }
                FqxValueType::I16 => {
                    table.col(cd.small_integer());
                }
                FqxValueType::I32 => {
                    table.col(cd.integer());
                }
                FqxValueType::I64 => {
                    table.col(cd.big_integer());
                }
                FqxValueType::F32 => {
                    table.col(cd.float());
                }
                FqxValueType::F64 => {
                    table.col(cd.double());
                }
                FqxValueType::String => {
                    table.col(cd.string());
                }
                FqxValueType::Blob => {
                    table.col(cd.binary());
                }
                FqxValueType::Null => {
                    table.col(cd.string());
                }
            }
        }

        table.to_owned()
    }

    fn create_tiberius_table(&self, table_name: &str) -> Result<String> {
        mssql_sqlbuild::create_table(&self, table_name)
    }

    fn drop_sqlx_table(&self, table_name: &str) -> TableDropStatement {
        Table::drop().table(Alias::new(table_name)).to_owned()
    }

    fn drop_tiberius_table(&self, table_name: &str) -> String {
        mssql_sqlbuild::drop_table(table_name)
    }

    fn sqlx_insert(self, table_name: &str) -> Result<InsertStatement> {
        let mut query = Query::insert();
        let columns = self
            .columns
            .iter()
            .map(|c| Alias::new(c))
            .collect::<Vec<_>>();
        query.into_table(Alias::new(table_name)).columns(columns);

        for row in self.data.into_iter() {
            query.values(
                row.0
                    .into_iter()
                    .map(|e| match e {
                        FqxValue::Bool(v) => v.into(),
                        FqxValue::U8(v) => v.into(),
                        FqxValue::U16(v) => v.into(),
                        FqxValue::U32(v) => v.into(),
                        FqxValue::U64(v) => v.into(),
                        FqxValue::I8(v) => v.into(),
                        FqxValue::I16(v) => v.into(),
                        FqxValue::I32(v) => v.into(),
                        FqxValue::I64(v) => v.into(),
                        FqxValue::F32(v) => v.into(),
                        FqxValue::F64(v) => v.into(),
                        FqxValue::String(v) => v.into(),
                        FqxValue::Blob(v) => v.into(),
                        FqxValue::Null => Option::<String>::None.into(), // Option type doesn't effect 'Null' value
                    })
                    .collect::<Vec<_>>(),
            )?;
        }

        Ok(query.to_owned())
    }

    fn tiberuis_insert(self, table_name: &str) -> Result<String> {
        Ok(mssql_sqlbuild::insert(self, table_name))
    }
}

// ================================================================================================
// Connector dyn fn
// ================================================================================================

impl Connector {
    pub async fn dyn_fetch(&self, sql: &str) -> Result<FqxData> {
        let mut proc = FqxSqlRowProcessor::new();

        let data = match self.db() {
            FqxPool::M(p) => {
                let stream = sqlx::query(sql)
                    .try_map(|r| proc.process_sqlx_row(r))
                    .fetch(p);

                stream.try_collect::<Vec<_>>().await?
            }
            FqxPool::P(p) => {
                let stream = sqlx::query(sql)
                    .try_map(|r| proc.process_sqlx_row(r))
                    .fetch(p);

                stream.try_collect::<Vec<_>>().await?
            }
            FqxPool::S(p) => {
                let stream = sqlx::query(sql)
                    .try_map(|r| proc.process_sqlx_row(r))
                    .fetch(p);

                stream.try_collect::<Vec<_>>().await?
            }
            FqxPool::Q(p) => {
                let mut pool = p.acquire().await?;

                let mut stream = pool.simple_query(sql).await?.into_row_stream();
                let mut data = vec![];

                while let Ok(Some(row)) = stream.try_next().await {
                    data.push(proc.process_tiberius_row(row)?);
                }

                data
            }
        };

        Ok(FqxData {
            columns: proc.columns().unwrap_or_default(),
            types: proc.types().unwrap_or_default(),
            data,
        })
    }

    pub async fn dyn_save(
        &self,
        mut data: FqxData,
        table_name: &str,
        mode: SaveMode,
        type_coercion: bool,
    ) -> Result<()> {
        // make sure each row has the same type series
        if type_coercion {
            data.type_coercion()?;
        }

        match mode {
            SaveMode::Override => {
                let (dt, ct) = match self.db() {
                    FqxPool::M(_) => {
                        let drop_table = data.drop_sqlx_table(table_name);
                        let create_table = data.create_sqlx_table(table_name);
                        (
                            drop_table.to_string(MysqlQueryBuilder),
                            create_table.to_string(MysqlQueryBuilder),
                        )
                    }
                    FqxPool::P(_) => {
                        let drop_table = data.drop_sqlx_table(table_name);
                        let create_table = data.create_sqlx_table(table_name);
                        (
                            drop_table.to_string(PostgresQueryBuilder),
                            create_table.to_string(PostgresQueryBuilder),
                        )
                    }
                    FqxPool::S(_) => {
                        let drop_table = data.drop_sqlx_table(table_name);
                        let create_table = data.create_sqlx_table(table_name);
                        (
                            drop_table.to_string(SqliteQueryBuilder),
                            create_table.to_string(SqliteQueryBuilder),
                        )
                    }
                    FqxPool::Q(_) => {
                        let drop_table = data.drop_tiberius_table(table_name);
                        let create_table = data.create_tiberius_table(table_name)?;
                        (drop_table, create_table)
                    }
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

fn _dyn_insert_data(db: &FqxPool, data: FqxData, table_name: &str) -> Result<String> {
    let res = match db {
        FqxPool::M(_) => data.sqlx_insert(table_name)?.to_string(MysqlQueryBuilder),
        FqxPool::P(_) => data
            .sqlx_insert(table_name)?
            .to_string(PostgresQueryBuilder),
        FqxPool::S(_) => data.sqlx_insert(table_name)?.to_string(SqliteQueryBuilder),
        FqxPool::Q(_) => data.tiberuis_insert(table_name)?,
    };

    Ok(res)
}
