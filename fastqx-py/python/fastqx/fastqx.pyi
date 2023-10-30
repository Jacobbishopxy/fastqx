# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Tuple, Union, Callable, Any, Dict, Optional
from dataclasses import dataclass
from enum import Enum

import pandas as pd

from .sql import FqxSqlConnector

# ================================================================================================
# General Types
# ================================================================================================

JsonType = Union[None, int, float, str, bool, List[JsonType], Dict[str, JsonType]]

FqxVT = Union[str, float, int, bytes, None]

GET_DATA_TYPE = Union[
    int,  # PyIdx::R
    slice,  # PyIdx::PS
    Tuple[int, int],  # PyIdx::V
    Tuple[slice, slice],  # PyIdx::RSS
    Tuple[int, slice],  # PyIdx::RIS
    Tuple[slice, int],  # PyIdx::RSI
]

SET_DATA_TYPE = Union[
    FqxVT,  # PyAssign::S
    List[FqxVT],  # PyAssign::D1
    List[List[FqxVT]],  # PyAssign::D2
]

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

    def is_float(self) -> bool: ...

# ================================================================================================
# FqxRow
# ================================================================================================

@dataclass
class FqxRow:
    row: List[FqxVT]

    # ================================================================================================
    # Py methods
    # ================================================================================================

    def __repr__(self) -> str: ...
    def __getitem__(self, idx: int) -> FqxVT: ...
    def __setitem__(self, idx: int, val: FqxVT): ...
    def __add__(self, rhs: FqxRow) -> FqxRow: ...
    def __sub__(self, rhs: FqxRow) -> FqxRow: ...
    def __mul__(self, rhs: FqxRow) -> FqxRow: ...
    def __truediv__(self, rhs: FqxRow) -> FqxRow: ...
    def __mod__(self, rhs: FqxRow) -> FqxRow: ...

    # ================================================================================================
    # Conversions
    # ================================================================================================

    def to_json(self) -> str: ...

# ================================================================================================
# FqxData
# ================================================================================================

@dataclass
class FqxData:
    columns: List[str]
    types: List[FqxValueType]
    data: List[List[FqxVT]]

    # ================================================================================================
    # Py methods
    # ================================================================================================

    def __repr__(self) -> str: ...
    def __getitem__(self, mtd: GET_DATA_TYPE) -> FqxData: ...
    #
    def __setitem__(self, mtd: GET_DATA_TYPE, val: SET_DATA_TYPE): ...
    #
    def __iter__(self) -> FqxData: ...
    def __next__(self) -> List[FqxVT]: ...

    # ================================================================================================
    # Helpers & Conversions
    # ================================================================================================

    def shape(self) -> Tuple[int, int]: ...
    def type_coercion(self): ...
    def cast(self, idx: int, typ: FqxValueType): ...
    def set_columns(self, columns: List[str]): ...
    @classmethod
    def from_list(cls, data: List[List[FqxVT]]) -> FqxData: ...
    def to_list(self) -> List[List[FqxVT]]: ...
    @classmethod
    def from_dict(cls, data: List[Dict[str, FqxVT]]) -> FqxData: ...
    def to_records(self) -> List[Dict[str, FqxVT]]: ...
    def to_dataframe(self) -> pd.DataFrame: ...
    def to_json(self) -> str: ...
    def to_json_pretty(self) -> str: ...
    @classmethod
    def from_csv(cls, path: str, type_hints: List[FqxValueType]) -> FqxData: ...
    def to_csv(self, path: str): ...
    def to_dataclass(self, dc: Callable[..., Any]) -> List[object]: ...
    @classmethod
    def from_sql(cls, sql: str, conn: FqxSqlConnector) -> FqxData: ...
    def to_sql(self, table: str, conn: FqxSqlConnector, mode: FqxSaveMode): ...

    # ================================================================================================
    # Ops
    # ================================================================================================

    def apply(self, fn: Callable[[FqxRow], Any]) -> List[JsonType]: ...
    def sum(self) -> Optional[FqxRow]: ...
    def min(self) -> Optional[FqxRow]: ...
    def max(self) -> Optional[FqxRow]: ...
    def mean(self) -> Optional[FqxRow]: ...
    def cum_sum(self) -> List[FqxRow]: ...
    def cum_min(self) -> List[FqxRow]: ...
    def cum_max(self) -> List[FqxRow]: ...
    def cum_mean(self) -> List[FqxRow]: ...
    def filter(self, fn: Callable[[FqxRow], bool]) -> FqxData: ...
    def reduce(self, fn: Callable[[FqxRow, FqxRow], FqxRow]) -> Optional[FqxRow]: ...
    # def x(self, x: SLICE_DATA_TYPE) -> FqxData: ...

def new_fqx_data(
    data: List[List[FqxVT]], columns: Optional[List[str]] = None
) -> FqxData: ...

# ================================================================================================
# FqxSaveMode
# ================================================================================================

class FqxSaveMode(Enum):
    Override = 1
    Append = 2
