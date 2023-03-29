from __future__ import annotations
from typing import Any
from typing_extensions import Self

__version__: str

def is_named_pattern(ptn: str) -> bool:
    ...

class FormatPatternTrait:
    def unformat(self, s: str) -> tuple[list[str], list[Any]]:
        ...

class FormatPattern(FormatPatternTrait):
    ...

class NamedFormatPattern(FormatPatternTrait):
    ...
