# @file:	test_data_dataframe.py
# @author:	Jacob Xie
# @date:	2023/09/21 22:03:04 Thursday
# @brief:


cols = ["c1", "c2", "c3"]
d = [[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, "a", 1.2]]


from fastqx import new_fqx_data

data = new_fqx_data(cols, d)

df = data.to_dataframe()

print(df)
