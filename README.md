# unformat

Python string unformatter.

Simple string pattern matching using f-string-like syntax.
This module cannot do anything more than `re` but is much easier to use.

### Installation

```shell
pip install -U unformat
```

### Usage

Unformatting returns a `list`-like object.

```Python
from unformat import compile, unformat

unformat("{name}_{idx}.csv", "data_001.csv")  # Values(name='data', idx='001')
unformat("{name}_{idx:int}.csv", "data_001.csv")  # Values(name='data', idx=1)
unformat("{name}_{idx}.csv", "data001.csv")  # Error!
name, idx = unformat("{name}_{idx}.csv", "data_001.csv")  # `Values` is list-like

# object oriented
ptn = compile("{name}_{idx}.csv")
ptn.unformat("data_001.csv")  # Values(name='data', idx='001')
```

Model-based unformatting is also supported.

```Python
from unformat import UnformatModel

class Version(UnformatModel):
    pattern = "{major}.{minor}.{micro}"  # not a field
    major: int
    minor: int
    micro: int

version = Version.from_string("0.2.1")  # Version(major=0, minor=2, micro=1)
```
