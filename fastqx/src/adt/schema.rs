//! file: schema.rs
//! author: Jacob Xie
//! date: 2023/10/10 11:05:37 Tuesday
//! brief:

use crate::adt::{FqxData, FqxRow, FqxValue, FqxValueType};

// ================================================================================================
// FqxSchema
// ================================================================================================

pub struct FqxSchema {
    pub columns: Vec<String>,
    pub types: Vec<FqxValueType>,
}

pub trait FqxSchemaGetter<R> {
    fn schema_len(&self) -> usize;

    fn get_schema(&self) -> FqxSchema;

    fn gen_empty_row(&self) -> R;
}

///////////////////////////////////////////////////////////////////////////////////////////////////

impl IntoIterator for FqxSchema {
    type Item = (String, FqxValueType);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns
            .into_iter()
            .zip(self.types.into_iter())
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Extend<(String, FqxValueType)> for FqxSchema {
    fn extend<T: IntoIterator<Item = (String, FqxValueType)>>(&mut self, iter: T) {
        let (c, t): (Vec<_>, Vec<_>) = iter.into_iter().unzip();
        self.columns.extend(c);
        self.types.extend(t);
    }
}

// ================================================================================================
// Impl
// ================================================================================================

impl FqxSchemaGetter<FqxRow> for FqxData {
    fn schema_len(&self) -> usize {
        self.width()
    }

    fn get_schema(&self) -> FqxSchema {
        FqxSchema {
            columns: self.columns.clone(),
            types: self.types.clone(),
        }
    }

    fn gen_empty_row(&self) -> FqxRow {
        FqxRow(vec![FqxValue::Null; self.width()])
    }
}

impl<'a> FqxSchemaGetter<FqxRow> for &'a FqxData {
    fn schema_len(&self) -> usize {
        self.width()
    }

    fn get_schema(&self) -> FqxSchema {
        FqxSchema {
            columns: self.columns.clone(),
            types: self.types.clone(),
        }
    }

    fn gen_empty_row(&self) -> FqxRow {
        FqxRow(vec![FqxValue::Null; self.width()])
    }
}

// ================================================================================================
// FqxDataGenenartor
// ================================================================================================

pub trait FqxDataGenerator<T> {
    fn from_d(data: T, schema: FqxSchema) -> Self;
}

impl<T, E> FqxDataGenerator<T> for FqxData
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
