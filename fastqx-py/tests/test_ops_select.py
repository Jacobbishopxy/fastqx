# @file:	test_ops_select.py
# @author:	Jacob Xie
# @date:	2023/10/28 13:55:50 Saturday
# @brief:

from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", 3.4], [4, "a", 1.2]],
)
print(data)

print(data.x(0))
print(data.x([0, 1]))
print(data.x(["c3", "c1"]))
print(data.x(slice(1, 2)))
print(data.x((slice(1), slice(1, 2))))
