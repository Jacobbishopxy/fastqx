# @file:	test_http.py
# @author:	Jacob Xie
# @date:	2023/10/02 21:46:08 Monday
# @brief:

from fastqx.http import FqxHttpConnector

url = "https://httpbin.org"

connector = FqxHttpConnector(url)

###################################################################################################

d = connector.get("get")
print(d)


req = {"foo": 1, "bar": [1.1, 2.1]}
d = connector.post("post", req)
print(d)
