# bgpkit-parser-py

Python binding for bgpkit-parser

## Example

```python
from pybgpkit_parser import Parser
import json

parser = Parser(
    url="https://spaces.bgpkit.org/parser/update-example",
    filters={"peer_ips": "185.1.8.65, 2001:7f8:73:0:3:fa4:0:1"},
)

for elem in parser:
    print(elem["origin_asns"])
    print(json.dumps(elem, indent=4))
    break
```

You can also add `cache_dir` to Parser to cache the downloaded files to a specified directory.

Here is an example:
```python
from pybgpkit_parser import Parser
import json

parser = Parser(
    url="https://spaces.bgpkit.org/parser/update-example",
    filters={"peer_ips": "185.1.8.65, 2001:7f8:73:0:3:fa4:0:1"},
    cache_dir="./"
)

for elem in parser:
    print(elem["origin_asns"])
    print(json.dumps(elem, indent=4))
    break
```

## Supported Python Version

- Python3.8
- Python3.9
- Python3.10
- Python3.11
- Python3.12

## Installation

```bash
python3 -m pip install pybgpkit-parser
```

## Develop

`maturin develop` builds local python module and add to the venv.

## Build and publish

See [BUILD.md](./BUILD.md) for more details.