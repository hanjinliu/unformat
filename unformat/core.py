from __future__ import annotations

from typing import Any, Callable, Sequence, Iterator, Union
from ._unformat_rust import is_named_pattern, FormatPattern, NamedFormatPattern

_RustFormatPattern = Union[FormatPattern, NamedFormatPattern]
_FMT_FUNCS: dict[str, Callable[[str], Any]] = {
    "int": int,
    "float": float,
    "str": str,
    "bool": bool,
    "complex": complex,
    "bytes": bytes,
    "bytearray": bytearray,
    "": lambda x: x,
}

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
            for k, v in self.items():
                args.append(f"{k}={v!r}")
            srepr = ", ".join(args)
        else:
            srepr = ", ".join(map(repr, self._values))
        return f"{cname}({srepr})"
    
    def items(self) -> Iterator[tuple[str, Any]]:
        if self._keys:
            for name, idx in self._keys.items():
                yield name, self._values[idx]
        else:
            raise ValueError("Values are not named")

    def asdict(self) -> dict[str, Any]:
        return dict(self.items())


class Pattern:
    def __init__(self, obj: _RustFormatPattern) -> None:
        self._rust_obj = obj
        self._fmts = [_FMT_FUNCS[f] for f in obj.formats()]
    
    def __repr__(self) -> str:
        cname = self.__class__.__name__
        return f"{cname}({self._rust_obj.pattern()!r})"
    
    def unformat(self, s: str) -> Values:
        """Unformat a string using the pattern."""
        keys, values = self._rust_obj.unformat(s)
        _vals = [fmt(v) for fmt, v in zip(self._fmts, values)]
        return Values(_vals, keys)
    
    def match(self, s: str) -> bool:
        """Check if the string matches the pattern."""
        return self._rust_obj.matches(s)


def compile(ptn: str) -> Pattern:
    if is_named_pattern(ptn):
        rust_obj = NamedFormatPattern(ptn)
    else:
        rust_obj = FormatPattern(ptn)
    return Pattern(rust_obj)

def unformat(ptn: str, s: str) -> Values:
    return compile(ptn).unformat(s)

def match(ptn: str, s: str) -> bool:
    return compile(ptn).match(s)
