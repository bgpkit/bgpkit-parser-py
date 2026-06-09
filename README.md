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

## Filters

The original dictionary-based filter API is still supported:

```python
parser = Parser(url, filters={"peer_ips": "185.1.8.65,2001:7f8:73:0:3:fa4:0:1"})
```

Reusable Rust-backed filters are also available:

```python
from pybgpkit_parser import Filter, Parser

filters = [
    Filter.peer_ips(["185.1.8.65", "2001:7f8:73:0:3:fa4:0:1"]),
    Filter.elem_type("a"),
]
parser = Parser.from_filters(url, filters)
```

Available helper constructors:

- `Filter.peer_ip(...)`
- `Filter.peer_ips([...])`
- `Filter.origin_asn(...)`
- `Filter.prefix(...)`
- `Filter.elem_type(...)`

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
        pub peer_bgp_id: Option<String>,
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
        #[pyo3(get, set)]
        pub only_to_customer: Option<u32>,
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

## High-performance projected iteration

For best performance, prefer projected tuple iteration when you only need a subset of fields. This avoids creating full `Elem` objects and skips conversion for unused fields.

```python
from pybgpkit_parser import Parser, ROUTE_FIELDS

# Fast: only converts requested fields
for timestamp, prefix, as_path in Parser(url).iter_tuples(["timestamp", "prefix", "as_path"]):
    pass

# Faster for large files: batch Python boundary crossings
fields = ["timestamp", "prefix", "as_path"]
for batch in Parser(url).iter_tuple_batches(fields, batch_size=10_000):
    for timestamp, prefix, as_path in batch:
        pass
```

Available field presets:

- `BASIC_FIELDS`: `timestamp`, `elem_type`, `peer_ip`, `peer_asn`, `prefix`
- `ROUTE_FIELDS`: `BASIC_FIELDS` + `as_path`
- `NEXT_HOP_FIELDS`: `BASIC_FIELDS` + `next_hop`

You can also pass your own field list, e.g. `Parser(url).iter_tuples(["peer_asn", "prefix"])`.

## Utility methods

`Elem` exposes Rust-like helper methods:

- `is_announcement()`
- `is_withdrawal()`
- `get_origin_asn()`
- `get_origin_asns()`
- `has_as_path()`
- `to_dict()` / `as_dict()`
- `origin_asn` property
- `to_json()`
- `to_psv()`
- `Elem.get_psv_header()`
- module constants: `ELEM_TYPE_ANNOUNCE`, `ELEM_TYPE_WITHDRAW`, `PSV_HEADER`

`Parser` also provides stream-consuming helpers:

- `count()`
- `iter_batches(batch_size)`
- `iter_tuples(fields)` — recommended for high-performance subset-field scans
- `iter_tuple_batches(fields, batch_size)` — recommended for large-file scans

## Route-level parsing

`RouteParser` exposes upstream `BgpRouteElem` iteration for faster scans when you only need route identity fields:

```python
from pybgpkit_parser import RouteParser

for route in RouteParser(url):
    print(route.timestamp, route.peer_ip, route.peer_asn, route.prefix, route.as_path)
```

`RouteElem` fields:

- `timestamp`
- `elem_type`
- `peer_ip`
- `peer_asn`
- `prefix`
- `as_path`

`RouteParser` supports the same constructor style, `from_filters(...)`, `parse_all()`, `parse_next()`, `count()`, `iter_batches(batch_size)`, `iter_tuples(fields)`, and `iter_tuple_batches(fields, batch_size)`.

For route scans, this is the fastest object-based API; for maximum throughput use `RouteParser.iter_tuples(ROUTE_FIELDS)` or `RouteParser.iter_tuple_batches(ROUTE_FIELDS, batch_size)`.

## Build and publish

See [BUILD.md](./BUILD.md) for automated GitHub Actions release details.