# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Any
from dataclasses import dataclass

@dataclass
class FqxData:
    columns: List[str]
    data: List[List[Any]]

class FqxConnector(object):
    def __init__(self, conn_str: str) -> None: ...
    def open(self) -> None: ...
    def close(self) -> None: ...
    def is_close(self) -> bool: ...
    async def execute(self, sql: str) -> None: ...
    async def fetch(self, sql: str) -> FqxData: ...
