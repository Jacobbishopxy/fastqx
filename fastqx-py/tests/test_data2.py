# @file:	test_data2.py
# @author:	Jacob Xie
# @date:	2023/09/19 14:49:00 Tuesday
# @brief:

from typing import Optional, List, Callable, Any
from dataclasses import dataclass
from fastqx import FqxData, new_fqx_data, FqxRow

###################################################################################################


data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, "a", 1.2]],
)

print(data[0][1])

data[0][1] = "foo"

print(data[0][1])


# @dataclass
# class RandomData:
#     c1: int
#     c2: str
#     c3: Optional[float]


# def to_dataclass(dc: Callable[..., Any], d: FqxData) -> List[object]:
#     return [dc(*row) for row in d.data]


# ###################################################################################################

# # foo = to_dataclass(RandomData, data)

# # print(foo)

# ###################################################################################################


# def create_dataclass_instances(dataclass_type: Callable[..., Any]):
#     def decorator(process_func: Callable[[FqxData], FqxData]):
#         def wrapper(d: FqxData) -> List[object]:
#             res = process_func(d)
#             return [dataclass_type(*item) for item in res.data]

#         return wrapper

#     return decorator


# @create_dataclass_instances(RandomData)
# def to_rand(d: FqxData):
#     print("d: ", d.to_json())

#     # d.data = []
#     for row in d.data:
#         if row[2] is not None:
#             # TODO: value assign is not allowed here
#             row[2] = row[2] * 3
#     return d


# print("to_rand: ", to_rand(data))
