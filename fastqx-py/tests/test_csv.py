# @file:	test_csv.py
# @author:	Jacob Xie
# @date:	2023/09/14 23:49:29 Thursday
# @brief:

from fastqx import FqxData, FqxValueType
from fastqx.csv import fqx_data_from_csv, fqx_data_to_csv

original = FqxData.from_list([[1, "a", "!"], [2, "b", "?"]])
original.to_csv("temp.csv")

data = FqxData.from_csv(
    "temp.csv",
    ["i16", "string", "string"],
)
print(data)

data = fqx_data_from_csv(
    "temp.csv",
    ["i16", "string", "string"],
)


print(data.to_str_pretty())

# data.to_csv("temp2.csv")
# print("finish to_csv")

fqx_data_to_csv(data, "temp2.csv")
