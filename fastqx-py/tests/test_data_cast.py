# @file:	test_data_cast.py
# @author:	Jacob Xie
# @date:	2023/09/23 17:58:16 Saturday
# @brief:

from fastqx import new_fqx_data, FqxValueType

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[
        [0, "s", 2.5],
        [1, "x", 2.3],
        [2, "y", 3.1],
        [3, "z", None],
        [4, "a", 1.2],
        [5, "q", 1.1],
        [6, "w", 3.0],
        [7, "e", 3.1],
        [8, "r", 2.5],
        [9, "t", 4.9],
    ],
)

print(data.types)

# cast col `c3` to i16 type
data.cast(2, "i16")

print(data)
