# @file:	test_data_slice.py
# @author:	Jacob Xie
# @date:	2023/09/21 13:17:43 Thursday
# @brief:

from fastqx import new_fqx_data, FqxRow


data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[
        [0, "s", 2.5],
        [1, "x", 2.3],
        [2, "y", 3.1],
        [3, "z", None],
    ],
)

###################################################################################################
# getter

print("getter:")

print("data[1] > ", data[1])
print("data[1:] > ", data[1:])
print("data[1, 2] > ", data[1, 2])
print("data[1:, 1:] > ", data[1:, 1:])
print("data[1, 1:] > ", data[1, 1:])
print("data[1:, 1] > ", data[1:, 1])

###################################################################################################
# setter

print("\nsetter:")

data[1] = [0, "s+", 2.51]
print("data[1] > ", data[1])
data[1] = FqxRow([0, "s++", 2.59])
print("data[1] > ", data[1])

data[1:] = [
    [1, "x+", 2.31],
    [2, "y+", 3.11],
    [3, "z+", 4.11],
]
print("data[1:] > ", data[1:])

data[1, 2] = "x+"
print("data[1, 2] > ", data[1, 2])

data[1:, 1:] = [["x+", 2.31], ["y+", 3.11], ["z+", 4.11]]
print("data[1:, 1:] > ", data[1:, 1:])

data[1, 1:] = ["x+", 2.31]
print("data[1, 1:] > ", data[1, 1:])

data[1:, 1] = ["x+", "y+", "z+"]
print("data[1:, 1] > ", data[1:, 1])
