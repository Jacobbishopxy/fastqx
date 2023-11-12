# @file:	test_data_objects.py
# @author:	Jacob Xie
# @date:	2023/09/23 08:19:15 Saturday
# @brief:

from fastqx import FqxData


raw = [
    {"a": 1, "b": 1.2, "c": "hah"},
    {
        "c": "z",
        "a": 2,
    },
    {
        "a": 13,
        "b": 2.2,
    },
    {
        "c": "eaz",
        "b": 3.1,
    },
]


data = FqxData.from_records(raw)
print(data)

objects = data.to_records()
print(objects)
