# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Optional, Tuple, Union
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

FqxVT = Union[str, float, int, bytes, None]

# ================================================================================================
# FqxRow
# ================================================================================================

class FqxRow:
    row: List[FqxVT]

    def to_json(self) -> Optional[str]: ...
    #
    def __repr__(self) -> str: ...
    def __getitem__(self, idx: int) -> FqxVT: ...
    def __setitem__(self, idx: int, val: FqxVT): ...

# ================================================================================================
# FqxData
# ================================================================================================

@dataclass
class FqxData:
    columns: List[str]
    types: List[FqxValueType]
    data: List[List[FqxVT]]

    def shape(self) -> Tuple[int, int]: ...
    def type_coercion(self): ...
    def to_list(self) -> List[List[FqxVT]]: ...
    def to_json(self) -> Optional[str]: ...
    def to_json_pretty(self) -> Optional[str]: ...
    @classmethod
    def from_csv(cls, path: str, type_hints: List[FqxValueType]) -> FqxData: ...
    def to_csv(self, path: str): ...
    #
    def __repr__(self) -> str: ...
    def __getitem__(self, idx: int) -> List[FqxVT]: ...
    def __setitem__(self, idx: int, val: List[FqxVT]): ...
    def __iter__(self) -> FqxData: ...
    def __next__(self) -> List[FqxVT]: ...

def new_fqx_data(columns: List[str], data: List[List[FqxVT]]) -> FqxData: ...
def fqx_data_from_csv(path: str, type_hints: List[FqxValueType]) -> FqxData: ...
def fqx_data_to_csv(data: FqxData, path: str): ...

# ================================================================================================
# FqxSaveMode
# ================================================================================================

class FqxSaveMode(Enum):
    Override = 1
    Append = 2

# ================================================================================================
# FqxConnectorConfig
# ================================================================================================

@dataclass
class FqxConnectorConfig:
    host: str
    port: int
    user: str
    pswd: str
    db: str
    extra: Optional[str]

# ================================================================================================
# FqxConnectorType
# ================================================================================================

class FqxDriver(Enum):
    MYSQL = 1
    POSTGRES = 2
    MSSQL = 3
    SQLITE = 4

    def to_conn_str(self, config: FqxConnectorConfig) -> str: ...

# ================================================================================================
# FqxConnector
# ================================================================================================

class FqxConnector(object):
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
