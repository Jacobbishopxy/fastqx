# @file:	sql.pyi
# @author:	Jacob Xie
# @date:	2023/09/21 23:56:02 Thursday
# @brief:

from typing import Optional
from dataclasses import dataclass
from enum import Enum

from .. import FqxData, FqxSaveMode

# ================================================================================================
# FqxConnectorConfig
# ================================================================================================

@dataclass
class FqxSqlConnectorConfig:
    host: str
    port: int
    user: str
    pswd: str
    db: str
    extra: Optional[str]

# ================================================================================================
# FqxConnectorType
# ================================================================================================

class FqxSqlDriver(Enum):
    MYSQL = 1
    POSTGRES = 2
    MSSQL = 3
    SQLITE = 4

    def to_conn_str(self, config: FqxSqlConnectorConfig) -> str: ...

# ================================================================================================
# FqxSqlConnector
# ================================================================================================

class FqxSqlConnector(object):
    def __init__(self, conn_str: str) -> None: ...
    def conn_str(self) -> str: ...
    def close(self): ...
    def is_close(self) -> bool: ...
    async def async_execute(self, sql: str): ...
    async def async_fetch(self, sql: str) -> FqxData: ...
    def execute(self, sql: str): ...
    def fetch(self, sql: str) -> FqxData: ...
    def save(self, data: FqxData, table_name: str, mode: FqxSaveMode): ...
    def uncheck_save(self, data: FqxData, table_name: str, mode: FqxSaveMode): ...
