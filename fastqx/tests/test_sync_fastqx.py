# @file:	test_sync_fastqx.py
# @author:	Jacob Xie
# @date:	2023/09/13 22:36:24 Wednesday
# @brief:


from fastqx import FqxConnector, new_fqx_data

conn_str = "postgres://dev:devpass@localhost:5437/dev"

connector = FqxConnector(conn_str)

print("is_close: ", connector.is_close())
connector.open()
print("is_close: ", connector.is_close())

data = new_fqx_data(["c1", "c2", "c3"], [[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]])

print("save table...")
connector.save(data, "tmp_table2", "override")
print("save complete")

res = connector.fetch("select * from tmp_table2")

print("res: \n", res.to_json_pretty())
