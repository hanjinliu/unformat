from __future__ import annotations
from typing import Any, Iterator, Sequence
from ._unformat_rust import is_named_pattern, FormatPattern, NamedFormatPattern

class Values(Sequence[Any]):
    def __init__(self, values: list[Any], keys: dict[str, int] | None = None) -> None:
        self._values = values
        self._keys = keys
    
    def __getitem__(self, key: int | str) -> Any:
        if isinstance(key, int):
            return self._values[key]
        elif isinstance(key, str):
            return self._values[self._keys[key]]
        raise TypeError(f"key must be int or str, not {type(key)}")

    def __len__(self) -> int:
        return len(self._values)
    
    def __iter__(self) -> Iterator[Any]:
        return iter(self._values)
    
    def __repr__(self) -> str:
        cname = self.__class__.__name__
        if self._keys:
            args = []
            for k, v in zip(self._keys.keys(), self._values):
                args.append(f"{k}={v!r}")
            srepr = ", ".join(args)
        else:
            srepr = ", ".join(map(repr, self._values))
        return f"{cname}({srepr})"

def unformat(ptn: str, s: str) -> Values:
    if is_named_pattern(ptn):
        keys, values = NamedFormatPattern(ptn).unformat(s)
    else:
        keys, values = FormatPattern(ptn).unformat(s)
    return Values(values, keys)
