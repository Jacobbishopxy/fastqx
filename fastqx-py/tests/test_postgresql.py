# @file:	test_postgresql.py
# @author:	Jacob Xie
# @date:	2023/09/13 22:36:24 Wednesday
# @brief:

from fastqx import new_fqx_data, FqxSaveMode, FqxData
from fastqx.sql import FqxSqlConnector

conn_str = "postgresql://dev:devpass@localhost:5437/dev"

connector = FqxSqlConnector(conn_str)

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    # vector of row
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]],
)

###################################################################################################
# method #1: using `connector`


print("save table...")
connector.save(data, "tmp_table2", FqxSaveMode.Override)
print("save complete")

res = connector.fetch("select * from tmp_table2")

print("res: \n", res.to_json_pretty())

###################################################################################################
# method #2: using `.to_sql` & `.from_sql`

data.to_sql("tmp_table3", connector, FqxSaveMode.Override)

res = FqxData.from_sql("select * from tmp_table3", connector)

print("res: \n", res.to_json_pretty())
