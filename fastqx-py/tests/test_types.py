# @file:	test_types.py
# @author:	Jacob Xie
# @date:	2023/09/15 22:06:58 Friday
# @brief:

from fastqx import FqxValueType


print(FqxValueType.Bool)
print(FqxValueType.I16)
print(FqxValueType.F64)
print(FqxValueType.String)
print(FqxValueType.Blob)

print(repr(FqxValueType.Bool))
print(repr(FqxValueType.I16))
print(repr(FqxValueType.F64))
print(repr(FqxValueType.String))
print(repr(FqxValueType.Blob))
