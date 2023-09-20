# @file:	helper.py
# @author:	Jacob Xie
# @date:	2023/09/20 23:35:57 Wednesday
# @brief:

from typing import List, Callable, Any
from . import FqxData


def to_dataclass(dc: Callable[..., Any], d: FqxData) -> List[object]:
    return [dc(*row) for row in d.data]


def create_dataclass_instances(dataclass_type: Callable[..., Any]):
    def decorator(process_func: Callable[[FqxData], FqxData]):
        def wrapper(d) -> List[object]:
            res = process_func(d.to_list())
            return [dataclass_type(*item) for item in res]

        return wrapper

    return decorator
