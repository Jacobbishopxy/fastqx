# @file:	test_ops_group.py
# @author:	Jacob Xie
# @date:	2023/11/12 21:02:22 Sunday
# @brief:


from fastqx import new_fqx_data

data = new_fqx_data(
    columns=["c1", "c2", "c3", "c4"],
    data=[
        [3, "x", "a", 2.3],
        [2, "y", "b", 3.1],
        [1, "x", "a", 3.1],
        [4, "zz", "b", 1.2],
        [2, "y", "b", 1.2],
        [1, "zz", "b", 1.2],
    ],
)


g = data.group_by(["c1", "c2"])
print(g[[3, "x"]])

for k, v in g.items():
    print("k: ", k)
    print("v: ", v)
    print("sum(v): ", v.sum())

for k in g.keys():
    print("k: ", k)

for v in g.values():
    print("v: ", v)


###################################################################################################


g = data.group_by(["c4"])
print(g[[1.2]])
print(g[[2.2]])  # None

for k, v in g.items():
    print("k: ", k)
    print("v: ", v)
    print("cum_mean(v): ", v.cum_mean())


for k in g.keys():
    print("k: ", k)

for v in g.values():
    print("v: ", v)
