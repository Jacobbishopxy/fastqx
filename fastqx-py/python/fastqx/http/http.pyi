# @file:	http.pyi
# @author:	Jacob Xie
# @date:	2023/10/02 19:04:20 Monday
# @brief:

from typing import Optional, Union, List, Dict

JsonType = Union[None, int, float, str, bool, List[JsonType], Dict[str, JsonType]]

# ================================================================================================
# FqxHttpConnector
# ================================================================================================

class FqxHttpConnector(object):
    def __init__(self, url: str, auth: Optional[str]) -> None: ...
    def url(self) -> str: ...
    def get(self, subpath: str) -> JsonType: ...
    def post(self, subpath: str, req: JsonType) -> JsonType: ...
