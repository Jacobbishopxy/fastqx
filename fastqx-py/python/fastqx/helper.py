# @file:	helper.py
# @author:	Jacob Xie
# @date:	2023/09/20 23:35:57 Wednesday
# @brief:

from typing import List, Callable, Any
from . import FqxData


def to_dataclass(dc: Callable[..., Any], d: FqxData) -> List[object]:
    return [dc(*row) for row in d.data]


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
