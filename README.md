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
    print(elem.origin_asns)
    print(json.dumps(elem.to_dict(), indent=4))
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
    print(elem.origin_asns)
    print(json.dumps(elem.to_dict(), indent=4))
    break
```

## Available fields for `Elem`

```rust
    #[pyclass]
    #[derive(Clone, PartialEq)]
    pub struct Elem {
        #[pyo3(get, set)]
        pub timestamp: f64,
        #[pyo3(get, set)]
        pub elem_type: String,
        #[pyo3(get, set)]
        pub peer_ip: String,
        #[pyo3(get, set)]
        pub peer_asn: u32,
        #[pyo3(get, set)]
        pub prefix: String,
        #[pyo3(get, set)]
        pub next_hop: Option<String>,
        #[pyo3(get, set)]
        pub as_path: Option<String>,
        #[pyo3(get, set)]
        pub origin_asns: Option<Vec<u32>>,
        #[pyo3(get, set)]
        pub origin: Option<String>,
        #[pyo3(get, set)]
        pub local_pref: Option<u32>,
        #[pyo3(get, set)]
        pub med: Option<u32>,
        #[pyo3(get, set)]
        pub communities: Option<Vec<String>>,
        #[pyo3(get, set)]
        pub atomic: Option<String>,
        #[pyo3(get, set)]
        pub aggr_asn: Option<u32>,
        #[pyo3(get, set)]
        pub aggr_ip: Option<String>,
    }
```

## Supported Python Version

- Python3.9
- Python3.10
- Python3.11
- Python3.12
- Python3.13

## Installation

```bash
python3 -m pip install pybgpkit-parser
```

## Develop

`maturin develop` builds local python module and add to the venv.

## Build and publish

See [BUILD.md](./BUILD.md) for more details.