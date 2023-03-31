# unformat

Python string unformatter.

Simple string pattern matching using f-string-like syntax.
This module cannot do anything more than `re` but is much easier to use.

### Usage

```Python
from unformat import unformat

unformat("{name}_{idx}.csv", "data_001.csv")  # Values(name='data', idx='001')
unformat("{name}_{idx:int}.csv", "data_001.csv")  # Values(name='data', idx=1)
unformat("{name}_{idx}.csv", "data001.csv")  # Error!
```
