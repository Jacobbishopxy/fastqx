# FastQX

A DataFetch lib written in Rust, works for both Rust and Python. Powered by [pyo3](https://github.com/PyO3/pyo3) & [maturin](https://github.com/PyO3/maturin).

Sources:

- [x] Sql (MsSql/MySql/Postgresql/Sqlite)
- [x] Csv
- [x] Http (Json)

Data Helper:

- [x] [owned](./fastqx/src/ops/owned.rs)
- [x] [idx](./fastqx/src/ops/idx.rs)

Data Operators:

- [x] [apply](./fastqx/src/ops/apply.rs)
- [x] [group](./fastqx/src/ops/group.rs)
- [x] [select](./fastqx/src/ops/select.rs)
- [x] [reduce](./fastqx/src/ops/reduce.rs)
- [x] [fold](./fastqx/src/ops/fold.rs)
- [x] [agg](./fastqx/src/ops/agg.rs)
- [x] [cum_agg](./fastqx/src/ops/cumagg.rs)
- [x] [compare](./fastqx/src/ops/compare.rs)
- [x] [filter](./fastqx/src/ops/filter.rs)
- [x] [sort](./fastqx/src/ops/sort.rs)
- [x] [position](./fastqx/src/ops/position.rs)
- [x] [merge](./fastqx/src/ops/merge.rs)
- [x] [join](./fastqx/src/ops/join.rs)
- [ ] pivot
- [ ] melt
- [ ] explode
- [ ] window
- [ ] rolling
- [ ] expand
- [ ] concat
- [ ] combine
- [ ] arithmetic

## Quick Start for Rust

Data construction:

```rs
use fastqx::fqx;

// create a `null` value
let a = fqx!();
println!("{:?}", a);

// create a value
let a = fqx!(1);
println!("{:?}", a);

// create an empty row
let a = fqx!(());
println!("{:?}", a);

// create a row
let a = fqx!(1, "ab", 2.0);
println!("{:?}", a);

// create a data
let a = fqx!(
    (1, "a", 1.1),
    (2, "b", None::<f32>),
    (3, "c", 3.3),
    (4, "d", 4.4),
    (5, "e", 5.5),
);
println!("{:?}", a);
```

Other test cases in Rust:

- [Dynamic Sql save & fetch](./fastqx/tests/sql_dynamic.rs)
- [Static Sql save & fetch](./fastqx/tests/sql_static.rs)

## Quick Start for Python

- [Sql query](./fastqx-py/tests/test_create_sql_query.py)
- [Csv read & write](./fastqx-py/tests/test_csv.py)
- [Data type casting](./fastqx-py/tests/test_data_cast.py)
- [Py DataClass conversions](./fastqx-py/tests/test_data_dataclass.py)
- [Pandas DataFrame conversions](./fastqx-py/tests/test_data_dataframe.py)
- [Py Dict conversions](./fastqx-py/tests/test_data_objects.py)
- [Py data constructions](./fastqx-py/tests/test_data.py)
- [Data slicing](./fastqx-py/tests/test_data_slice.py)
- [Http curl](./fastqx-py/tests/test_http.py)
- [MsSql connection](./fastqx-py/tests/test_mssql_conn.py)
- [MsSql save & fetch](./fastqx-py/tests/test_mssql.py)
- [Data operation: agg/cum_agg](./fastqx-py/tests/test_ops_agg.py)
- [Data operation: apply](./fastqx-py/tests/test_ops_apply.py)
- [Data operation: filter](./fastqx-py/tests/test_ops_filter.py)
- [Data operation: group](./fastqx-py/tests/test_ops_group.py)
- [Data operation: join](./fastqx-py/tests/test_ops_join.py)
- [Data operation: merge](./fastqx-py/tests/test_ops_merge.py)
- [Data operation: reduce](./fastqx-py/tests/test_ops_reduce.py)
- [Postgresql connection](./fastqx-py/tests/test_postgresql_conn.py)
- [Postgresql save & fetch](./fastqx-py/tests/test_postgresql.py)
- [Sql type](./fastqx-py/tests/test_sql_conn.py)
- [Data type](./fastqx-py/tests/test_types.py)

## Dev

- Install Python dependencies: `make devenv-init`

- Build Python package: `make install`
