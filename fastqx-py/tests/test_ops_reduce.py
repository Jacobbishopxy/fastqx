# @file:	test_ops_reduce.py
# @author:	Jacob Xie
# @date:	2023/10/27 09:52:28 Friday
# @brief:

from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", 3.4], [4, "a", 1.2]],
)
print(data)

res = data.reduce(lambda p, c: p + c)
print(res)

res = data.reduce(lambda p, c: p * c)
print(res)
