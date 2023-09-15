# @file:	types.py
# @author:	Jacob Xie
# @date:	2023/09/12 21:51:47 Tuesday
# @brief:


from enum import Enum


class NoValue(Enum):
    def __repr__(self) -> str:
        return "<%s.%s>" % (self.__class__.__name__, self.name)


class FqxValueType(NoValue):
    Bool = "Bool"
    U8 = "U8"
    U16 = "U16"
    U32 = "U32"
    U64 = "U64"
    I8 = "I8"
    I16 = "I16"
    I32 = "I32"
    I64 = "I64"
    F32 = "F32"
    F64 = "F64"
    String = "String"
    Blob = "Blob"
    Null = "Null"
