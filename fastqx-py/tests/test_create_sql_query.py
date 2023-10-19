# @file:	test_create_sql_query.py
# @author:	Jacob Xie
# @date:	2023/10/19 15:05:55 Thursday
# @brief:

from fastqx import create_sql_query, TypeT
from fastqx.sql import FqxSqlConnector

conn_str = "postgresql://dev:devpass@localhost:5437/dev"

connector = FqxSqlConnector(conn_str)


@create_sql_query(connector)
def select_astrisk(table: str):
    return f"select * from {table}"


res = select_astrisk("tmp_table")

print(res(TypeT.Fqx))
print(res(TypeT.Df))
print(res(TypeT.Json))
print(res(TypeT.Record))
