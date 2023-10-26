# @file:	test_ops_filter.py
# @author:	Jacob Xie
# @date:	2023/10/26 15:57:03 Thursday
# @brief:


from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", 3.4], [4, "a", 1.2]],
)
print(data)

res = data.filter(lambda r: r[0] in [2, 3])
print(res)
