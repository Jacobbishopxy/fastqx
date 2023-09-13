# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Optional, Any
from dataclasses import dataclass

@dataclass
class FqxData:
    columns: List[str]
    types: List[str]
    data: List[List[Any]]

    def type_coercion(self) -> None: ...
    def to_json(self) -> Optional[str]: ...
    def to_json_pretty(self) -> Optional[str]: ...

def new_fqx_data(columns: List[str], data: List[List[Any]]) -> FqxData: ...

class FqxConnector(object):
    def __init__(self, conn_str: str) -> None: ...
    def open(self) -> None: ...
    def close(self) -> None: ...
    def is_close(self) -> bool: ...
    async def execute(self, sql: str) -> None: ...
    async def fetch(self, sql: str) -> FqxData: ...
