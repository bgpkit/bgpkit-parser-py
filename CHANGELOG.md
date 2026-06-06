# Changelog

All notable changes to this project will be documented in this file.

## 0.7.0 - TBD

### Highlights

* Update `bgpkit-parser` to v0.17.0.
* Update PyO3 to v0.28 and enable ABI3 wheels for Python 3.9+.
* Add `peer_bgp_id` and `only_to_customer` fields to `Elem`.
* Add reusable `Filter` class and `Parser.from_filters(...)` constructor.
* Add Rust-like `Elem` utility methods: `is_announcement`, `is_withdrawal`, `get_origin_asn`, `get_origin_asns`, `has_as_path`, `as_dict`, `to_json`, `to_psv`, and `get_psv_header`.
* Add `Elem.origin_asn` property and module constants `ELEM_TYPE_ANNOUNCE`, `ELEM_TYPE_WITHDRAW`, and `PSV_HEADER`.
* Add Python-native filter helper constructors: `Filter.peer_ip`, `Filter.peer_ips`, `Filter.origin_asn`, `Filter.prefix`, and `Filter.elem_type`.
* Add stream-consuming `Parser.count()` and `Parser.iter_batches(batch_size)` helpers.
* Add `RouteElem` and `RouteParser` for upstream route-level parsing (`BgpRouteElem`) and faster route identity scans.
* Add high-performance projected tuple iteration: `iter_tuples(fields)` and `iter_tuple_batches(fields, batch_size)` for `Parser` and `RouteParser`.
* Add field presets `BASIC_FIELDS`, `ROUTE_FIELDS`, and `NEXT_HOP_FIELDS`.
* Optimize `Parser.parse_all()` and batch iteration by parsing while detached from the Python interpreter before converting results into Python objects.
* Add Rust and Python benchmark scaffolding.
* Automate wheel builds and PyPI publishing via GitHub Actions.

## 0.6.2 - 2025-06-06

### Fix regressions

* Fix a regression in the `Elem` class that the default string representation was missing
* Fix a regression that the `Parser` class requires `filters` and `cache_dir` to be passed in the constructor,
  which should be and was optional priority to `v0.6.0`.

## 0.6.1 - 2025-06-06

### Highlights

* Update `bgpkit-parser` to v0.11.1, which includes a fix on parsing for `next_hop` for IPv6 peers.

## 0.6.0 - 2025-06-04

### Highlights

* Update `bgpkit-parser` to v0.11.0, which includes several bug fixes and performance improvements.
* Add support for Python 3.13.

### Breaking changes

* The `Elem` class's fields can only be access by their getter methods now. Direct access to fields is no longer
  allowed. This change improves encapsulation and ensures that the internal state of `Elem` is managed correctly.

## 0.5.1 - 2024-02-28

### Highlights

* update `bgpkit-parser` to v0.10.1, which fixes a performance regression introduced in 0.10.0.

