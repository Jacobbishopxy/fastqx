# @file:	test_postgres.py
# @author:	Jacob Xie
# @date:	2023/09/13 22:36:24 Wednesday
# @brief:

from fastqx import new_fqx_data
from fastqx.sql import FqxSqlConnector, FqxSaveMode

conn_str = "postgres://dev:devpass@localhost:5437/dev"

connector = FqxSqlConnector(conn_str)

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    # vector of row
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]],
)

print("save table...")
connector.save(data, "tmp_table2", FqxSaveMode.Override)
print("save complete")

res = connector.fetch("select * from tmp_table2")

print("res: \n", res.to_json_pretty())
