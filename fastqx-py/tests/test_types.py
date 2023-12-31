# @file:	test_types.py
# @author:	Jacob Xie
# @date:	2023/09/15 22:06:58 Friday
# @brief:

import datetime as dt
from fastqx import FqxValueType, FqxRow


print(FqxValueType.Bool)
print(FqxValueType.U8)
print(FqxValueType.U16)
print(FqxValueType.U32)
print(FqxValueType.U64)
print(FqxValueType.I8)
print(FqxValueType.I16)
print(FqxValueType.I32)
print(FqxValueType.I64)
print(FqxValueType.F32)
print(FqxValueType.F64)
print(FqxValueType.String)
print(FqxValueType.Blob)
print(FqxValueType.Null)

_d = dt.date(2023, 11, 12)
_t = dt.time(6, 50, 10)
_dt = dt.datetime(2023, 11, 12, 6, 50, 10)

row = FqxRow([_d, _t, _dt])
print(row)
print(row.types())
