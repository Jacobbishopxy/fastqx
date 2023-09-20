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


@dataclass
class RandomData:
    c1: int
    c2: str
    c3: Optional[float]


d = to_dataclass(RandomData, data)
print(d)

print(data.to_list())

###################################################################################################


@create_dataclass_instances(RandomData)
def to_rand(d: FqxData):
    # for loop is clone `d.data`
    for row in d:
        if row[2] is not None:
            row[2] = row[2] * 3

    return d


print("to_rand: ", to_rand(data))
