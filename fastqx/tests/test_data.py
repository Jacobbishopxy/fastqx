# @file:	test_data.py
# @author:	Jacob Xie
# @date:	2023/09/12 20:41:58 Tuesday
# @brief:

from fastqx import FqxData, FqxValueType


data = FqxData(
    columns=["c1", "c2", "c3"],
    types=[FqxValueType.I32.value, FqxValueType.String.value, FqxValueType.F32.value],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]],
)

print(data.types)
print(data.to_json())
