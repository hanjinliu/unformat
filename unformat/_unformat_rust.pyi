from __future__ import annotations
from typing_extensions import Self

def parse(s: str):
    ...

class FormatPatternTrait:
    def from_string(cls, s: str) -> Self:
        ...
    def parse(self, s: str):
        ...

class FormatPattern(FormatPatternTrait):
    ...

class NamedFormatPattern(FormatPatternTrait):
    ...
