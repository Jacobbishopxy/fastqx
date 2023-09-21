# @file:	test_data2.py
# @author:	Jacob Xie
# @date:	2023/09/19 14:49:00 Tuesday
# @brief:

from typing import Optional
from dataclasses import dataclass
from fastqx import FqxData, new_fqx_data, to_dataclass, create_dataclass_instances

###################################################################################################


data = new_fqx_data(
    columns=["c1", "c2", "c3"],
    data=[[1, "x", 2.3], [2, "y", 3.1], [3, "z", None], [4, "a", 1.2]],
)

print(data.to_list())


@dataclass
class RandomData:
    c1: int
    c2: str
    c3: Optional[float]


d = to_dataclass(RandomData, data)
print(d)

d = data.to_dataclass(RandomData)
print(d)

###################################################################################################


# `create_dataclass_instances` will first turn `FqxData` into python lists
@create_dataclass_instances(RandomData)
def to_rand(d: FqxData):
    # for loop is clone `d.data`
    for idx, row in enumerate(d):
        print(row)
        if row[2] is not None:
            row[2] = row[2] * 3
        d[idx] = row

    return d


print("to_rand: ", to_rand(data))
