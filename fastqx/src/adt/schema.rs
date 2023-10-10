//! file: schema.rs
//! author: Jacob Xie
//! date: 2023/10/10 11:05:37 Tuesday
//! brief:

use crate::adt::{FqxData, FqxRow, FqxValueType};

// ================================================================================================
// FqxSchema
// ================================================================================================

pub struct FqxSchema {
    pub columns: Vec<String>,
    pub types: Vec<FqxValueType>,
}

pub trait FqxSchemaGetter {
    fn get_schema(&self) -> FqxSchema;
}

// ================================================================================================
// Impl
// ================================================================================================

impl FqxSchemaGetter for FqxData {
    fn get_schema(&self) -> FqxSchema {
        FqxSchema {
            columns: self.columns.clone(),
            types: self.types.clone(),
        }
    }
}

impl<'a> FqxSchemaGetter for &'a FqxData {
    fn get_schema(&self) -> FqxSchema {
        FqxSchema {
            columns: self.columns.clone(),
            types: self.types.clone(),
        }
    }
}

// ================================================================================================
// FqxDataGenenartor
// ================================================================================================

pub trait FqxDataGenenartor<T> {
    fn from_d(data: T, schema: FqxSchema) -> Self;
}

impl<T, E> FqxDataGenenartor<T> for FqxData
where
    T: IntoIterator<Item = E>,
    FqxRow: From<E>,
{
    fn from_d(data: T, schema: FqxSchema) -> Self {
        FqxData {
            columns: schema.columns,
            types: schema.types,
            data: data.into_iter().map(FqxRow::from).collect(),
        }
    }
}
