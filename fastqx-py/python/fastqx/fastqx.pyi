# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Optional, Any
from dataclasses import dataclass
from enum import Enum

# ================================================================================================
# FqxValueType
# ================================================================================================

class FqxValueType(Enum):
    Bool = 1
    U8 = 2
    U16 = 3
    U32 = 4
    U64 = 5
    I8 = 6
    I16 = 7
    I32 = 8
    I64 = 9
    F32 = 10
    F64 = 11
    String = 12
    Blob = 13
    Null = 14

# ================================================================================================
# FqxData
# ================================================================================================

@dataclass
class FqxData:
    columns: List[str]
    types: List[FqxValueType]
    data: List[List[Any]]

    def type_coercion(self) -> None: ...
    def to_json(self) -> Optional[str]: ...
    def to_json_pretty(self) -> Optional[str]: ...
    @classmethod
    def from_csv(cls, path: str, type_hints: List[str]) -> FqxData: ...
    def to_csv(self, path: str) -> None: ...

def new_fqx_data(columns: List[str], data: List[List[Any]]) -> FqxData: ...
def fqx_data_from_csv(path: str, type_hints: List[str]) -> FqxData: ...
def fqx_data_to_csv(data: FqxData, path: str) -> None: ...

# ================================================================================================
# FqxConnectorType
# ================================================================================================

class FqxConnectorType(Enum):
    MySql = 1
    Postgres = 2
    MsSql = 3
    Sqlite = 4

# ================================================================================================
# FqxConnector
# ================================================================================================

class FqxConnector(object):
    def __init__(self, conn_str: str) -> None: ...
    def open(self) -> None: ...
    def close(self) -> None: ...
    def is_close(self) -> bool: ...
    async def async_execute(self, sql: str) -> None: ...
    async def async_fetch(self, sql: str) -> FqxData: ...
    def execute(self, sql: str) -> None: ...
    def fetch(self, sql: str) -> FqxData: ...
    def save(self, data: FqxData, table_name: str, mode: str) -> None: ...
    def uncheck_save(self, data: FqxData, table_name: str, mode: str) -> None: ...
