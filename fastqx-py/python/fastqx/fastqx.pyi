# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Optional, Tuple, Union, Callable, Any
from dataclasses import dataclass
from enum import Enum

import pandas as pd

# ================================================================================================
# Const
# ================================================================================================

GET_DATA_TYPE = Union[int, Tuple[int, int], slice]
SET_DATA_TYPE = Union[FqxVT, List[FqxVT], List[List[FqxVT]]]

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
    @classmethod
    def from_list(cls, data: List[List[FqxVT]]) -> FqxData: ...
    def to_list(self) -> List[List[FqxVT]]: ...
    def to_dataframe(self) -> pd.DataFrame: ...
    def to_json(self) -> Optional[str]: ...
    def to_json_pretty(self) -> Optional[str]: ...
    @classmethod
    def from_csv(cls, path: str, type_hints: List[FqxValueType]) -> FqxData: ...
    def to_csv(self, path: str): ...
    def to_dataclass(self, dc: Callable[..., Any]) -> List[object]: ...
    #
    def __repr__(self) -> str: ...
    def __getitem__(self, mtd: GET_DATA_TYPE) -> SET_DATA_TYPE: ...
    def __setitem__(self, mtd: GET_DATA_TYPE, val: SET_DATA_TYPE): ...
    def __iter__(self) -> FqxData: ...
    def __next__(self) -> List[FqxVT]: ...

def new_fqx_data(columns: List[str], data: List[List[FqxVT]]) -> FqxData: ...

# ================================================================================================
# FqxSaveMode
# ================================================================================================

class FqxSaveMode(Enum):
    Override = 1
    Append = 2
