# @file:	http.pyi
# @author:	Jacob Xie
# @date:	2023/10/02 19:04:20 Monday
# @brief:

from typing import Optional
from enum import Enum

from ..fastqx import JsonType, FqxData

# ================================================================================================
# FqxHttpMethod
# ================================================================================================

class FqxHttpMethod(Enum):
    Get = 1
    Post = 2
    Put = 3
    Delete = 4
    Patch = 5
    #
    def __repr__(self) -> str: ...

# ================================================================================================
# FqxHttpConnector
# ================================================================================================

class FqxHttpConnector(object):
    def __init__(self, url: str, auth: Optional[str] = None) -> None: ...
    def url(self) -> str: ...
    def get(self, subpath: str) -> JsonType: ...
    def post(self, subpath: str, req: JsonType) -> JsonType: ...
    def put(self, subpath: str, req: JsonType) -> JsonType: ...
    def delete(self, subpath: str) -> JsonType: ...
    def patch(self, subpath: str, req: JsonType) -> JsonType: ...
    def curl(
        self, subpath: str, method: FqxHttpMethod, payload: Optional[JsonType]
    ) -> FqxData: ...
