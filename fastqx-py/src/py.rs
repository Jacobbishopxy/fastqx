//! file: py.rs
//! author: Jacob Xie
//! date: 2023/09/10 19:49:32 Sunday
//! brief:

use pyo3::prelude::*;

use crate::csv::{fqx_data_from_csv, fqx_data_to_csv};
use crate::data::new_fqx_data;
use crate::sql::PyConnector;
use crate::{ConnectorConfig, Driver, FqxData, FqxRow, FqxValueType, SaveMode};

// ================================================================================================
// PyModule
// ================================================================================================

#[pymodule]
fn fastqx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SaveMode>()?;
    m.add_class::<Driver>()?;
    m.add_class::<ConnectorConfig>()?;
    m.add_class::<PyConnector>()?;
    m.add_class::<FqxValueType>()?;
    m.add_class::<FqxRow>()?;
    m.add_class::<FqxData>()?;
    m.add_wrapped(wrap_pyfunction!(new_fqx_data))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_from_csv))?;
    m.add_wrapped(wrap_pyfunction!(fqx_data_to_csv))?;

    Ok(())
}
