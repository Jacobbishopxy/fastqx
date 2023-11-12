# @file:	fastqx.pyi
# @author:	Jacob Xie
# @date:	2023/09/09 14:59:01 Saturday
# @brief:

from typing import List, Tuple, Union, Callable, Any, Dict, Optional
from dataclasses import dataclass
from enum import Enum

import pandas as pd
import datetime as dt

from .sql import FqxSqlConnector

# ================================================================================================
# General Types
# ================================================================================================

JsonType = Union[None, int, float, str, bool, List[JsonType], Dict[str, JsonType]]

FqxVT = Union[str, float, int, bytes, dt.date, dt.time, dt.datetime, None]

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
    List[FqxRow],  # PyAssign::RS
    FqxRow,  # PyAssign::R
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
    Timestamp = 14
    DateTime = 15
    Date = 16
    Time = 17
    Null = 18

    #
    def is_float(self) -> bool: ...

    #
    def is_numeric(self) -> bool: ...

# ================================================================================================
# FqxRow
# ================================================================================================

@dataclass
class FqxRow:
    row: List[FqxVT]

    # ================================================================================================
    # Py methods
    # ================================================================================================

    #
    def __repr__(self) -> str: ...

    #
    def __str__(self) -> str: ...

    #
    def __getitem__(self, idx: int) -> FqxVT: ...

    #
    def __setitem__(self, idx: int, val: FqxVT): ...

    #
    def __add__(self, rhs: FqxRow) -> FqxRow: ...

    #
    def __sub__(self, rhs: FqxRow) -> FqxRow: ...

    #
    def __mul__(self, rhs: FqxRow) -> FqxRow: ...

    #
    def __truediv__(self, rhs: FqxRow) -> FqxRow: ...

    #
    def __mod__(self, rhs: FqxRow) -> FqxRow: ...

    #
    def __len__(self) -> int: ...

    # to str
    def to_str(self) -> str: ...

    # cast
    def cast(self, idx: int, typ: FqxValueType): ...

    # types
    def types(self) -> List[FqxValueType]: ...

# ================================================================================================
# FqxData
# ================================================================================================

@dataclass
class FqxData:
    columns: List[str]
    types: List[FqxValueType]
    data: List[FqxRow]

    # ================================================================================================
    # Py methods
    # ================================================================================================

    #
    def __repr__(self) -> str: ...

    #
    def __getitem__(self, mtd: GET_DATA_TYPE) -> FqxData: ...

    #
    def __setitem__(self, mtd: GET_DATA_TYPE, val: SET_DATA_TYPE): ...

    #
    def __iter__(self) -> FqxData: ...

    #
    def __next__(self) -> FqxRow: ...

    #
    def __len__(self) -> int: ...

    # ================================================================================================
    # Data manipulations
    # ================================================================================================

    # length of data
    def height(self) -> int: ...

    # length of row
    def width(self) -> int: ...

    # 2d shape
    def shape(self) -> Tuple[int, int]: ...

    # push a row to the end of data
    def push(self, row: FqxRow): ...

    # extend rows to the end of data
    def extend(self, rows: List[FqxRow]): ...

    # insert a row into data
    def insert(self, idx: int, row: FqxRow): ...

    # pop the last row from data
    def pop(self) -> Optional[FqxRow]: ...

    # remove a row from data
    def remove(self, idx: int) -> Optional[FqxRow]: ...

    # retain rows by a lambda
    def retain(self, fn: Callable[[FqxRow], bool]): ...

    # reverse data
    def reverse(self): ...

    # ================================================================================================
    # Type casting
    # ================================================================================================

    # cast all value to match column types
    def type_coercion(self): ...

    # cast a column of data into another type
    def cast(self, idx: int, typ: FqxValueType): ...

    # ================================================================================================
    # Sources conversions
    # ================================================================================================

    # 2d
    @classmethod
    def from_list(cls, data: List[List[FqxVT]]) -> FqxData: ...
    def to_list(self) -> List[List[FqxVT]]: ...

    # list of dicts
    @classmethod
    def from_records(cls, data: List[Dict[str, FqxVT]]) -> FqxData: ...
    def to_records(self) -> List[Dict[str, FqxVT]]: ...

    # pandas DataFrame
    def to_dataframe(self) -> pd.DataFrame: ...

    # dataclass
    def to_dataclasses(self, dc: Callable[..., Any]) -> List[object]: ...

    # str
    def to_str(self) -> str: ...
    def to_str_pretty(self) -> str: ...

    # json (same as `json.dumps`)
    def to_json(self) -> Any: ...
    def to_json_records(self) -> Any: ...

    # csv
    @classmethod
    def from_csv(cls, path: str, type_hints: List[FqxValueType]) -> FqxData: ...
    def to_csv(self, path: str): ...

    # sql
    @classmethod
    def from_sql(cls, sql: str, conn: FqxSqlConnector) -> FqxData: ...
    def to_sql(self, table: str, conn: FqxSqlConnector, mode: FqxSaveMode): ...

    # ================================================================================================
    # X
    # ================================================================================================

    @property
    def x(self) -> X: ...

    # ================================================================================================
    # Ops
    # ================================================================================================

    #
    def apply(self, fn: Callable[[FqxRow], Any]) -> List[JsonType]: ...

    #
    def sum(self) -> Optional[FqxRow]: ...

    #
    def min(self) -> Optional[FqxRow]: ...

    #
    def max(self) -> Optional[FqxRow]: ...

    #
    def mean(self) -> Optional[FqxRow]: ...

    #
    def cum_sum(self) -> List[FqxRow]: ...

    #
    def cum_min(self) -> List[FqxRow]: ...

    #
    def cum_max(self) -> List[FqxRow]: ...

    #
    def cum_mean(self) -> List[FqxRow]: ...

    #
    def filter(self, fn: Callable[[FqxRow], bool]) -> FqxData: ...

    #
    def reduce(self, fn: Callable[[FqxRow, FqxRow], FqxRow]) -> Optional[FqxRow]: ...

    #
    def group_by(self, keys: List[str]) -> Dict[List[FqxVT], FqxData]: ...

    #
    def sort_by(self, fn: Callable[[FqxRow], bool]) -> FqxData: ...

    # merge
    # how: "left"/"right"/"outer"/"inner"
    def merge(
        self,
        other: FqxData,
        left_on: List[str],
        right_on: List[str],
        how: str,
    ) -> FqxData: ...

    # join
    # how: "left"/"right"/"outer"/"inner"
    def join(
        self,
        other: FqxData,
        on: List[str],
        how: str,
    ) -> FqxData: ...

#
def new_fqx_data(
    data: List[List[FqxVT]], columns: Optional[List[str]] = None
) -> FqxData: ...

# ================================================================================================
# X
# ================================================================================================

class X:
    #
    def __repr__(self) -> str: ...

    #
    def __str__(self) -> str: ...

    #
    def __getitem__(self, idx: GET_DATA_TYPE) -> List[List[FqxVT]]: ...

    #
    def __setitem__(self, mtd: GET_DATA_TYPE, val: SET_DATA_TYPE): ...

# ================================================================================================
# FqxSaveMode
# ================================================================================================

class FqxSaveMode(Enum):
    Override = 1
    Append = 2
