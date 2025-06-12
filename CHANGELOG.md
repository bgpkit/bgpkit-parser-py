# Changelog

All notable changes to this project will be documented in this file.

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

