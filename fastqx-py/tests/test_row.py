# @file:	test_row.py
# @author:	Jacob Xie
# @date:	2023/11/12 18:10:25 Sunday
# @brief:

from fastqx import FqxRow

foo = FqxRow([1, 0, "a", 2.1, 21])

print(foo)

print(foo[0])
print(foo[1])
print(foo[2])
print(foo[3])
print(foo[4])

foo[0] = "z"
foo[1] = "z"
foo[2] = "z"
foo[3] = "z"
foo[4] = "z"

print(foo)
