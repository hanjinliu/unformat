# unformat

Python string unformatter.

## Examples

###### 1. Simplest unformatting

```Python
from unformat import unformat

unformat(
    pattern="path/to/{date}-{index}.txt",
    string="path/to/221116-1.tif"
)
```

```
{"date": "221116", "index": "1"}
```

###### 2. Parse string

```Python
from unformat import unformat

unformat(
    pattern="path/to/{date:str}-{index:int}.tif",
    string="path/to/221116-1.tif"
)
```

```
{"date": "221116", "index": 1}
```

###### 3. Construct Unformatter from a dataclass

```Python
from dataclasses import dataclass
from unformat import construct

@dataclass
class A:
    date: str
    index: int

unformatter = construct(A, "path/to/{}-{}.txt")
unformatter("path/to/221116-1.tif")
```

```
A(date="221116", index=1)
```
