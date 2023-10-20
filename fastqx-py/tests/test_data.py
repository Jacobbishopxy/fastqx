# @file:	test_data.py
# @author:	Jacob Xie
# @date:	2023/09/12 20:41:58 Tuesday
# @brief:

from fastqx import FqxData, FqxValueType, new_fqx_data

# 1. derictly construct a `FqxData`

data = FqxData(
    columns=["c1", "c2", "c3"],
    types=[FqxValueType.I32, FqxValueType.String, FqxValueType.F32],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None]],
)

print(data.types)
print(data.to_json())

# type_coercion
# Since turning Python data type into Rust data type is based on `impl FromPyObject for FqxValue`,
# we need explicit call `.type_coercion` to make type.
# In this case, column "c1": I64 -> I32, column "c3": F64 -> F32
data.type_coercion()


# 2. construct a `FqxData` by `new_fqx_data` function

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, "a", 1.2]],
)

print(data.types)
print(data.to_json())
print(data.to_records())
