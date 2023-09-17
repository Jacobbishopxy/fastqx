# @file:	test_async_fastqx.py
# @author:	Jacob Xie
# @date:	2023/09/11 21:37:00 Monday
# @brief:

import logging
import asyncio

from fastqx import FqxConnector

# ================================================================================================
# Log
# ================================================================================================

FORMAT = "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
logging.basicConfig(
    level=logging.INFO,
    format=FORMAT,
    handlers=[
        #
        logging.FileHandler("debug.log"),
        logging.StreamHandler(),
    ],
)

# ================================================================================================
# Test
# ================================================================================================

conn_str = "postgres://dev:devpass@localhost:5437/dev"

connector = FqxConnector(conn_str)

print("is_close: ", connector.is_close())


loop = asyncio.get_event_loop()


async def fetch():
    sql = "select * from users"
    return await connector.async_fetch(sql)


d = loop.run_until_complete(fetch())

print("d.columns: \n", d.columns)
print("d.data: \n", d.data)

print("d.to_json(): \n", d.to_json())
print("d.to_json_pretty(): \n", d.to_json_pretty())

loop.close()
