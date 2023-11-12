# @file:	helper.py
# @author:	Jacob Xie
# @date:	2023/09/20 23:35:57 Wednesday
# @brief:

from typing import List, Callable, Any

from . import FqxData
from .sql import FqxSqlConnector


def create_dataclass_instances(dataclass_type: Callable[..., Any]):
    """convert a processed FqxData into a list of dataclass

    Args:
        dataclass_type (Callable[..., Any]): dataclass type
    """

    def decorator(process_func: Callable[[FqxData], FqxData]):
        def wrapper(d: FqxData) -> List[object]:
            proc_res = process_func(d)
            dataclass_list = proc_res.to_dataclasses(dataclass_type)
            return dataclass_list

        return wrapper

    return decorator


###################################################################################################


def create_sql_query(connector: FqxSqlConnector):
    def decorator(process_func) -> Callable[..., FqxData]:
        def wrapper(*args, **kwargs) -> FqxData:
            query_str = process_func(*args, **kwargs)
            return connector.fetch(query_str)

        return wrapper

    return decorator


def create_sql_exec(connector: FqxSqlConnector):
    def decorator(process_func) -> Callable[..., None]:
        def wrapper(*args, **kwargs) -> None:
            execute_str = process_func(args, kwargs)
            connector.execute(execute_str)

        return wrapper

    return decorator
