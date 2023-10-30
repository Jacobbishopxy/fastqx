# @file:	test_data_x.py
# @author:	Jacob Xie
# @date:	2023/10/30 22:04:42 Monday
# @brief:

from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, "a", 1.2]],
)

x = data.x

print(x)

print(x[1])

print(x[1:])

print(x[1, 2])

print(x[1:, 1:])

print(x[1, 1:])

print(x[1:, 1])
