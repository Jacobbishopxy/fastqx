# @file:	test_fastqx_conn.py
# @author:	Jacob Xie
# @date:	2023/09/15 21:57:07 Friday
# @brief:


from fastqx.sql import FqxSqlDriver

print(FqxSqlDriver.MSSQL)
print(FqxSqlDriver.MYSQL)
print(FqxSqlDriver.POSTGRES)
print(FqxSqlDriver.SQLITE)
