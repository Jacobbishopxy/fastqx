# @file:	test_data_csv.py
# @author:	Jacob Xie
# @date:	2023/09/14 23:49:29 Thursday
# @brief:

from fastqx import FqxData, FqxValueType


data = FqxData.from_csv(
    "temp.csv",
    [FqxValueType.I16, FqxValueType.String, FqxValueType.String],
)

print(data.to_json_pretty())

data.to_csv("temp2.csv")
print("finish to_csv")
