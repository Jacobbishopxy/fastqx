# @file:	test_mssql.py
# @author:	Jacob Xie
# @date:	2023/09/19 08:41:13 Tuesday
# @brief:

from fastqx import new_fqx_data
from fastqx.sql import FqxSqlConnector, FqxSaveMode

conn_str = "jdbc:sqlserver://localhost:1433;username=dev;password=StrongPassword123;databaseName=devdb"

connector = FqxSqlConnector(conn_str)

data = new_fqx_data(
    columns=["col1", "col2", "col3"],
    # vector of row
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]],
)

print("save table...")
connector.save(data, "tmp_table2", FqxSaveMode.Override)
print("save complete")

res = connector.fetch("select * from tmp_table2")

print("res: \n", res.to_json_pretty())
