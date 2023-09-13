//! file: dyn.rs
//! author: Jacob Xie
//! date: 2023/09/13 15:23:06 Wednesday
//! brief:

use anyhow::Result;
use futures::TryStreamExt;

use crate::adt::*;
use crate::sql::*;

// TODO: impl `ConnectorStatement` for `RoughData`?

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

    // TODO
    pub async fn dyn_save(&self, mut data: RoughData, mode: SaveMode) -> Result<()> {
        data.type_coercion()?;

        todo!()
    }
}
