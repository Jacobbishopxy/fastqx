# @file:	test_data_slice.py
# @author:	Jacob Xie
# @date:	2023/09/21 13:17:43 Thursday
# @brief:

from fastqx import new_fqx_data


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


print(data[2])
print(data[2, 1])
print(data[1:3])
print(data[1:-2])
print(data[-6:-2])
