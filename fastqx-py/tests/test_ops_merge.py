# @file:	test_ops_merge.py
# @author:	Jacob Xie
# @date:	2023/11/12 20:48:44 Sunday
# @brief:

from fastqx import new_fqx_data

data1 = new_fqx_data(
    columns=["c1", "c2", "c3", "c4"],
    data=[
        [1, "x", "a", 2.3],
        [2, "y", "a", 3.1],
        [3, "x", "b", 3.4],
        [4, "zz", "b", 1.2],
        [5, "y", "b", 1.2],
        [6, "zz", "bb", 1.2],
    ],
)

# TODO: bug if "a2", "a3" columns' position reversed
data2 = new_fqx_data(
    columns=["a1", "a2", "a3"],
    data=[
        [10, "x", "b"],
        [20, "y", "b"],
        [30, "x", "a"],
        [40, "y", "a"],
        [50, "z", "a"],
        [50, "z", "b"],
    ],
)

ans = data1.merge(data2, left_on=["c2", "c3"], right_on=["a2", "a3"], how="left")
print("left\n", ans, "\n")

ans = data1.merge(data2, left_on=["c2", "c3"], right_on=["a2", "a3"], how="right")
print("right\n", ans, "\n")

ans = data1.merge(data2, left_on=["c2", "c3"], right_on=["a2", "a3"], how="inner")
print("inner\n", ans, "\n")

ans = data1.merge(data2, left_on=["c2", "c3"], right_on=["a2", "a3"], how="outer")
print("outer\n", ans, "\n")
