# @file:	test_mssql_conn.py
# @author:	Jacob Xie
# @date:	2023/10/18 09:02:21 Wednesday
# @brief:

import os
from pathlib import Path
from dotenv import load_dotenv

from fastqx.sql import FqxSqlConnector

_ENV_DIR = Path(__file__).resolve().parent
load_dotenv(dotenv_path=f"{_ENV_DIR}/.env")

MS_SQL_CONN = os.getenv("MS_SQL_CONN")
if MS_SQL_CONN is None:
    raise FileNotFoundError(".env file is missing or `MS_SQL_CONN` is not set")

connector = FqxSqlConnector(MS_SQL_CONN)

res = connector.fetch("select 1")

print("res: \n", res.to_json_pretty())
