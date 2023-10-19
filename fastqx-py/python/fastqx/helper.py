# @file:	helper.py
# @author:	Jacob Xie
# @date:	2023/09/20 23:35:57 Wednesday
# @brief:

from typing import List, Callable, Dict, Union, Any
from enum import Enum

import pandas as pd

from . import FqxData
from .sql import FqxSqlConnector


def to_dataclass(dc: Callable[..., Any], d: FqxData) -> List[object]:
    return [dc(*row) for row in d]


def create_dataclass_instances(dataclass_type: Callable[..., Any]):
    """convert a processed FqxData into a list of dataclass

    Args:
        dataclass_type (Callable[..., Any]): dataclass type
    """

    def decorator(process_func: Callable[[FqxData], FqxData]):
        def wrapper(d: FqxData) -> List[object]:
            proc_res = process_func(d)
            dataclass_list = proc_res.to_dataclass(dataclass_type)
            return dataclass_list

        return wrapper

    return decorator


###################################################################################################


PyType = Union[FqxData, pd.DataFrame, str, List[Dict[str, Any]], None]


class TypeT(Enum):
    Fqx = 1
    Df = 2
    Json = 3
    Record = 4

    def into(self, d: FqxData) -> PyType:
        if self.value == 1:
            return d
        elif self.value == 2:
            return d.to_dataframe()
        elif self.value == 3:
            return d.to_json()
        elif self.value == 4:
            return d.to_dict()
        else:
            return None


def create_sql_query(connector: FqxSqlConnector):
    def decorator(process_func) -> Callable[..., Callable[[TypeT], PyType]]:
        def wrapper(*args, **kwargs) -> Callable[[TypeT], PyType]:
            query_str = process_func(*args, **kwargs)
            res = connector.fetch(query_str)

            return lambda t: TypeT.into(t, res)

        return wrapper

    return decorator


def create_sql_exec(connector: FqxSqlConnector):
    def decorator(process_func) -> Callable[..., None]:
        def wrapper(*args, **kwargs) -> None:
            execute_str = process_func(args, kwargs)
            connector.execute(execute_str)

        return wrapper

    return decorator
