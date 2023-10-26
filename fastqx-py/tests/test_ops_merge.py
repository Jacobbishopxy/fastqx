# @file:	test_ops_merge.py
# @author:	Jacob Xie
# @date:	2023/10/25 13:35:25 Wednesday
# @brief:

import pandas as pd

df1 = pd.DataFrame(
    {
        "Fruit": ["Pear", "Apple", "Pear", "Banana"],
        "Phosphorus (mg/100g)": [11, 22, 12, 15],
    }
)

df2 = pd.DataFrame(
    {
        "Name": ["Apple", "Pear", "Pear", "Pineapple", "Pear"],
        "Potassium (mg/100g)": [107, 358, 115, 200, 116],
    }
)

df_left = df1.merge(df2, "left", left_on=["Fruit"], right_on=["Name"])
print("df_left:")
print(df_left)

df_right = df1.merge(df2, "right", left_on=["Fruit"], right_on=["Name"])
print("df_right:")
print(df_right)

df_inner = df1.merge(df2, "inner", left_on=["Fruit"], right_on=["Name"])
print("df_inner:")
print(df_inner)

df_outer = df1.merge(df2, "outer", left_on=["Fruit"], right_on=["Name"])
print("df_outer:")
print(df_outer)
