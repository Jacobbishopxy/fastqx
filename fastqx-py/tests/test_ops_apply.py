# @file:	test_ops_apply.py
# @author:	Jacob Xie
# @date:	2023/10/09 11:14:37 Monday
# @brief:


from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, None, 1.2]],
)

res = data.apply(lambda row: [f'{row[1] or ""} !', row[2] or 0 * 2])
print(res)
