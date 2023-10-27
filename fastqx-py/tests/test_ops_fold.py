# @file:	test_ops_fold.py
# @author:	Jacob Xie
# @date:	2023/10/27 09:12:21 Friday
# @brief:

items = [1, 2, 3, 4, 5]

f = lambda acc, x: acc * x

accumulator = 1

res = [accumulator := f(accumulator, x) for x in items]

print(res)
print(accumulator)
