# @file:	csv.py
# @author:	Jacob Xie
# @date:	2023/09/22 08:35:05 Friday
# @brief:

from typing import List
from .. import FqxValueType, FqxData

def fqx_data_from_csv(path: str, type_hints: List[FqxValueType]) -> FqxData: ...
def fqx_data_to_csv(data: FqxData, path: str): ...
