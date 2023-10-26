# @file:	test_ops_agg.py
# @author:	Jacob Xie
# @date:	2023/10/09 13:17:41 Monday
# @brief:


from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", 3.4], [4, "a", 1.2]],
)
print(data)

print("sum: ", data.sum())
print("min: ", data.min())
print("max: ", data.max())
print("mean: ", data.mean())

print("cum_sum: ", data.cum_sum())
print("cum_min: ", data.cum_min())
print("cum_max: ", data.cum_max())
print("cum_mean: ", data.cum_mean())
