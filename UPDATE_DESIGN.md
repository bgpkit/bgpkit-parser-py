# bgpkit-parser-py Update Plan — Single PR

## Overview

This plan covers a comprehensive update of `bgpkit-parser-py` in **one pull request**. The PR modernizes dependencies, expands the Python API surface, adds performance benchmarks, and replaces the manual release process with fully automated CI/CD.

## Work Streams (4 in 1 PR)

| # | Stream | Scope | Risk |
|---|--------|-------|------|
| 1 | **Bump `bgpkit-parser`** | `0.11.1` → `0.17.0` | Medium — API changes, new fields |
| 2 | **Bump PyO3** | `0.25` → `0.28.3` | Medium — `Bound` API migration |
| 3 | **Expand Python API** | New fields, `RouteParser`, projected tuple iteration, `Filter` helpers, `__repr__`, `from_filters`, `count`, `iter_batches`, constants | Low — additive changes |
| 4 | **Benchmarks + CI/CD** | Python benchmark script, `criterion`, `maturin-action` workflow | Low — new files, no Rust API changes |

## Design Decisions

### 1. PyO3 Version: 0.28.3 (not 0.29)

**Decision:** Target PyO3 0.28.3.

**Rationale:** Latest stable release. 0.29 has breaking `pyo3-build-config` changes that require a direct dependency on `pyo3` or `pyo3-ffi`. 0.28.3 gives us free-threaded Python support and the mature `Bound` API without extra migration risk.

### 2. `Bound` API Migration

**Decision:** Migrate Python-object helpers to the `Bound<'py, T>` API where appropriate (e.g., `to_dict`), but keep `Py<Elem>` for returned element objects where it remains the clean PyO3 return type.

**Rationale:** PyO3 0.28 deprecates the old GIL-ref API. The `Bound` API is now primary. However, `Py::new()` still returns `Py<T>` which works fine. We migrate incrementally — what compiles cleanly with `Bound` gets converted; what doesn't stays on `Py`.

### 3. `Elem` Field Access: Keep `#[pyo3(get, set)]`

**Decision:** Retain direct field access on `Elem`.

**Rationale:** v0.6.0 was a breaking change that removed getters and caused user pain. The current `get, set` approach is ergonomic for Python users. No change.

### 4. New Fields: `peer_bgp_id` and `only_to_customer`

**Decision:** Add both to the Python `Elem` class.

**Rationale:**
- `peer_bgp_id`: PEER_INDEX_TABLE in TableDumpV2/RIB records. `Option<String>`.
- `only_to_customer`: RFC 9234 OTC attribute. `Option<u32>`. `None` for withdrawals (fixed upstream in v0.16.0).

### 5. Iterator Strategy: Expose Elem and Route Iterators, Defer Fallible/Record/Update

**Decision:** Keep `Parser` for `BgpElem` iteration and add `RouteParser` for upstream `BgpRouteElem` route-level iteration. Defer fallible, record, raw-record, and update iterators.

**Rationale:** `BgpRouteElem` is a compact route identity type and maps cleanly to Python (`RouteElem`). It is the best performance-oriented iterator to expose now. Fallible/record/update iterators return different result/enum/nested record types and should be added in follow-up PRs with more API design.

### 6. Filter API: Add `Filter` Class + `from_filters` Constructor

**Decision:** Keep the existing `HashMap<String, String>` filter constructor. Add a new `Filter` PyO3 class and a `from_filters` classmethod on `Parser`.

**Rationale:** The string-based API is backward-compatible. The new `Filter` class exposes the upstream `Filter::new()` constructor, and `from_filters` lets users pass pre-built `Filter` objects (enabling reuse and avoiding string parsing overhead).

### 7. Performance Benchmark: `pytest-benchmark` + `criterion`

**Decision:** Add a Python-side benchmark script and Rust-side (`criterion`) benchmarks.

**Rationale:** We need to quantify the "Python tax" per element. The benchmark compares:
- Rust native iteration (baseline)
- Python `for elem in parser` iteration (GIL crossing + object allocation)
- Python `parse_all` (bulk collection)
- Python `to_dict()` overhead (serialization)

Also optimize `parse_all()` by parsing while detached from the Python interpreter (`py.detach(...)`) before converting the collected Rust `Elem` values into Python objects.

### 8. CI/CD: `maturin-action` + PyPI Trusted Publishing

**Decision:** Use `PyO3/maturin-action` for all wheel builds, publish via PyPI Trusted Publishing (OIDC), and use ABI3 (`abi3-py39`) to build one wheel per platform instead of per Python version.

**Rationale:**
- `maturin-action` handles cross-compilation, manylinux, and all platforms automatically
- Replaces the manual process (2 Macs + Docker + `twine upload`)
- Produces ABI3 wheels for macOS x86_64/arm64, Linux x86_64/aarch64, Windows x86_64
- Trusted Publishing avoids long-lived PyPI API tokens
- Manual `workflow_dispatch` runs are build-only by default to avoid accidental PyPI publication

### 9. `unsafe impl Send/Sync` for `Parser`

**Decision:** Keep and verify after the `bgpkit-parser` bump.

**Rationale:** The `ElemIterator` type may have changed in v0.17.0. If `BgpkitParser::into_iter()` no longer returns `Send`, we switch to a different approach (e.g., `into_elem_iter()` or `into_fallible_elem_iter()`).

## Data Structures

### Python `Elem` (updated)

```rust
#[pyclass]
#[derive(Clone, PartialEq, Serialize)]
pub struct Elem {
    #[pyo3(get, set)] pub timestamp: f64,
    #[pyo3(get, set)] pub elem_type: String,
    #[pyo3(get, set)] pub peer_ip: String,
    #[pyo3(get, set)] pub peer_asn: u32,
    #[pyo3(get, set)] pub prefix: String,
    #[pyo3(get, set)] pub next_hop: Option<String>,
    #[pyo3(get, set)] pub as_path: Option<String>,
    #[pyo3(get, set)] pub origin_asns: Option<Vec<u32>>,
    #[pyo3(get, set)] pub origin: Option<String>,
    #[pyo3(get, set)] pub local_pref: Option<u32>,
    #[pyo3(get, set)] pub med: Option<u32>,
    #[pyo3(get, set)] pub communities: Option<Vec<String>>,
    #[pyo3(get, set)] pub atomic: Option<String>,
    #[pyo3(get, set)] pub aggr_asn: Option<u32>,
    #[pyo3(get, set)] pub aggr_ip: Option<String>,
    // NEW in v0.17
    #[pyo3(get, set)] pub peer_bgp_id: Option<String>,
    #[pyo3(get, set)] pub only_to_customer: Option<u32>,
}
```

### Python `RouteElem` (new)

```rust
#[pyclass]
pub struct RouteElem {
    #[pyo3(get, set)] pub timestamp: f64,
    #[pyo3(get, set)] pub elem_type: String,
    #[pyo3(get, set)] pub peer_ip: String,
    #[pyo3(get, set)] pub peer_asn: u32,
    #[pyo3(get, set)] pub prefix: String,
    #[pyo3(get, set)] pub as_path: Option<String>,
}
```

### Python `Filter` (new)

```rust
#[pyclass]
pub struct Filter {
    inner: bgpkit_parser::parser::Filter,
}

#[pymethods]
impl Filter {
    #[new]
    #[pyo3(signature = (filter_type, filter_value))]
    fn new(filter_type: String, filter_value: String) -> PyResult<Self> {
        let inner = bgpkit_parser::parser::Filter::new(filter_type.as_str(), filter_value.as_str())
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Filter { inner })
    }
}
```

## Single-PR Implementation Plan

### Phase 1: Dependency Bump + Compile

**1. `Cargo.toml`**
```toml
[package]
name = "pybgpkit-parser"
version = "0.7.0"
# ... rest unchanged ...

[dependencies]
bgpkit-parser = "0.17.0"
pyo3 = { version = "0.28", features = ["extension-module", "abi3-py39"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"

[build-dependencies]
pyo3-build-config = "0.28"
```

**2. `build.rs`**
```rust
fn main() {
    pyo3_build_config::add_extension_module_link_args();
}
```
No changes — `add_extension_module_link_args()` is stable in 0.28.

**3. `pyproject.toml`**
```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "pybgpkit-parser"
version = "0.7.0"
description = "Python binding for bgpkit-parser"
readme = "README.md"
license = { text = "MIT" }
requires-python = ">=3.9"
classifiers = [
    "Programming Language :: Rust",
    "Programming Language :: Python :: 3.9",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
    "Programming Language :: Python :: 3.13",
]
```

**4. Run `cargo check`**
- Fix compile errors iteratively
- Check if `ElemIterator` type changed in v0.17.0
- Verify `unsafe impl Send` still compiles

### Phase 2: `src/lib.rs` Rewrite

**5. `convert_elem` update**
- Map new fields: `peer_bgp_id`, `only_to_customer`
- Keep all existing field mappings

**6. `Elem` struct update**
- Add `peer_bgp_id: Option<String>`
- Add `only_to_customer: Option<u32>`
- Keep all existing `#[pyo3(get, set)]` fields

**7. `Elem::to_dict` update**
- Add `"peer_bgp_id"` and `"only_to_customer"` entries

**8. `Elem` utility methods (new)**
- `is_announcement()`
- `is_withdrawal()`
- `get_origin_asn()`
- `get_origin_asns()`
- `has_as_path()`
- `as_dict()`
- `origin_asn` property
- `to_json()`
- `to_psv()`
- `get_psv_header()`

**9. `Elem::__repr__` (new)**
```rust
#[pyo3(name = "__repr__")]
fn repr(&self) -> PyResult<String> {
    Ok(format!("<Elem prefix={} peer={} type={}>", self.prefix, self.peer_ip, self.elem_type))
}
```

**10. `Filter` class (new)**
- `#[pyclass]` wrapper around `bgpkit_parser::parser::Filter`
- `#[new]` constructor: `Filter(filter_type, filter_value)`
- Python-native helper constructors: `peer_ip`, `peer_ips`, `origin_asn`, `prefix`, `elem_type`
- Expose to the module

**11. `Parser` struct update**
- Keep `elem_iter: ElemIterator<Box<dyn Send + Read>>`
- Verify `Send` bound after v0.17.0 bump

**12. `Parser::new` (keep signature)**
```rust
#[pyo3(signature = (url, filters=None, cache_dir=None))]
fn new(
    url: String,
    filters: Option<HashMap<String, String>>,
    cache_dir: Option<String>,
) -> PyResult<Self>
```
Keep backward-compatible `HashMap<String, String>` filter API.

**13. `Parser::from_filters` (new, classmethod)**
```rust
#[staticmethod]
#[pyo3(signature = (url, filters, cache_dir=None))]
fn from_filters(
    url: String,
    filters: Vec<Bound<Filter>>,
    cache_dir: Option<String>,
) -> PyResult<Self> { ... }
```
- Construct `BgpkitParser` from URL
- Add filters via `with_filters` or `add_filters` using the `.inner` of each `Filter`
- Return `Parser` with the iterator

**14. `Parser::parse_all` (optimize)**
- Keep `Vec<Py<Elem>>` return type
- Use `py.detach(...)` to parse and convert `BgpElem` → Rust `Elem` outside the Python interpreter, then reacquire Python only to allocate Python objects

**15. `Parser::parse_next` (error-safe allocation)**
- Return `PyResult<Option<Py<Elem>>>`
- Avoid `.unwrap()` on Python allocation

**16. `Parser::__next__` (error-safe allocation)**
- Return `PyResult<Option<Py<Elem>>>`
- Avoid `.unwrap()` on Python allocation

**17. Parser and RouteParser utility methods + module registration**
- Add `Parser.count()` (stream-consuming)
- Add `Parser.iter_batches(batch_size)` returning `BatchIterator`
- Add `RouteElem` and `RouteParser` for upstream `into_route_iter()`
- Add `RouteParser.count()`, `RouteParser.parse_all()`, `RouteParser.parse_next()`, and `RouteParser.iter_batches(batch_size)`
- Add projected tuple iteration: `iter_tuples(fields)` and `iter_tuple_batches(fields, batch_size)` for `Parser` and `RouteParser`
- Register `Elem`, `RouteElem`, `Filter`, `Parser`, `BatchIterator`, tuple iterators, `RouteParser`, and `RouteBatchIterator`
- Add module constants: `ELEM_TYPE_ANNOUNCE`, `ELEM_TYPE_WITHDRAW`, `PSV_HEADER`, `BASIC_FIELDS`, `ROUTE_FIELDS`, `NEXT_HOP_FIELDS`

### Phase 3: Benchmarks

**17. `benches/parse_bench.rs` (new)**
- `criterion` benchmark for Rust native iteration
- `[[bench]]` entry in `Cargo.toml`

**18. `tests/benchmark.py` (new)**
- `time.perf_counter` benchmark
- Compares: `parse_all`, `for elem in parser`, `iter_batches`, projected tuple iteration, `RouteParser` equivalents, `to_dict()`
- Uses `https://spaces.bgpkit.org/parser/update-example` as test data

**19. `tests/test_api.py` (new)**
- Test `Filter` construction
- Test `Parser.from_filters`
- Test new `Elem` fields (`peer_bgp_id`, `only_to_customer`)
- Test `__repr__` and `__str__`
- Test `to_dict()` contains all fields

### Phase 4: CI/CD

**20. `.github/workflows/release.yml` (full rewrite)**
- Replace the current simple format-check + create-release workflow
- Add: `build-sdist` job, `build-wheels` matrix job, `publish-pypi` job
- Use `PyO3/maturin-action@v1`
- Platform matrix: macOS x86_64, macOS arm64, Linux x86_64, Linux aarch64, Windows x86_64
- ABI3 Python compatibility: 3.9–3.13 from one wheel per platform
- Trigger: `v[0-9]+.*` tags + `workflow_dispatch`
- PyPI publish via PyPI Trusted Publishing / GitHub OIDC (`id-token: write`)
- GitHub Release creation via `taiki-e/create-gh-release-action@v1`

**21. `.github/workflows/rust.yaml`**
- Keep as-is (format + clippy on PR/push)

**22. `BUILD.md` (rewrite)**
- Document the automated release process (push tag → CI builds + publishes)
- Keep the manual Docker/`build.sh` process as fallback documentation

**23. `CHANGELOG.md` (add entry)**
- v0.7.0: dependency bump, new fields, `Filter` class, benchmarks, CI/CD automation

**24. `README.md` (update)**
- Update `Elem` field list to show new fields
- Document `Filter` class and `from_filters` constructor
- Update installation notes

### Phase 5: Test + Verify

**25. Local testing**
- `maturin develop`
- Run `examples/filter_count_print.py`
- Run `tests/benchmark.py`
- Run `tests/test_api.py`

**26. CI testing**
- Open PR — `rust.yaml` runs format + clippy
- Verify no regressions

**27. Post-merge release test**
- Run `workflow_dispatch` with `publish=false` to verify build-only release workflow
- Verify all artifacts build successfully
- Push the real `v0.7.0` tag only when ready to publish to PyPI

## Changes to Existing Files (Summary)

| File | Change |
|------|--------|
| `Cargo.toml` | Bump `bgpkit-parser` → `0.17.0`, `pyo3` → `0.28` with `abi3-py39`, `pyo3-build-config` → `0.28`, add `[[bench]]` |
| `src/lib.rs` | Full rewrite: new fields, `RouteParser`, projected tuple iteration, `Filter` helpers, `from_filters`, `__repr__`, `count`, `iter_batches`, constants, `Bound` API migration |
| `pyproject.toml` | Add `project` metadata, `requires-python`, classifiers |
| `BUILD.md` | Rewrite to document CI workflow; keep manual fallback |
| `CHANGELOG.md` | Add v0.7.0 entry |
| `README.md` | Update API docs, new fields, `Filter` class |
| `.github/workflows/release.yml` | Full rewrite: add build matrix + PyPI publish + GitHub Release |

## New Files (Summary)

| File | Purpose |
|------|---------|
| `benches/parse_bench.rs` | Rust `criterion` benchmark for native iteration |
| `tests/benchmark.py` | Python benchmark comparing `parse_all` vs iteration vs `to_dict` |
| `tests/test_api.py` | Python tests for new API (`Filter`, new fields, `from_filters`) |

## Pre-PR Checklist

- [ ] PyPI Trusted Publisher configured for `bgpkit/bgpkit-parser-py`, workflow `release.yml`, environment `pypi`
- [ ] `cargo check` passes after dependency bump
- [ ] `maturin develop` builds successfully
- [ ] `examples/filter_count_print.py` runs without error
- [ ] `tests/test_api.py` passes
- [ ] `tests/benchmark.py` runs and produces meaningful numbers
- [ ] `benches/parse_bench.rs` compiles and runs
- [ ] `cargo fmt` passes
- [ ] `cargo clippy -- -D warnings` passes

## Post-Merge Checklist

- [ ] Run release workflow manually with `publish=false`
- [ ] Verify all artifacts are built but not published
- [ ] Verify GitHub Release is created with changelog
- [ ] If beta works, delete tag and push `v0.7.0`
- [ ] Update `AGENTS.md` with new anti-patterns (CI token handling, etc.)

## Open Questions

1. **Should we expose `BgpRouteElem` / `into_route_iter`?**
   - **Decision:** Yes. Exposed as `RouteElem` and `RouteParser` because it maps cleanly and directly supports performance comparisons.

2. **Should we expose `MrtRecord` / `into_record_iter`?**
   - **Default:** No. Adds complexity to the `Parser` struct (different iterator types). Follow-up PR.

3. **Should we add Windows to the CI matrix?**
   - **Decision:** Yes. `maturin-action` supports it out of the box. No platform-specific Rust code.

4. **Should we build `universal2` macOS wheels instead of separate x86_64/arm64?**
   - **Default:** No. Separate wheels are smaller. Revisit if users complain.

5. **Should we migrate to Trusted Publishing (OIDC) instead of API token?**
   - **Decision:** Yes. Release publishing uses `pypa/gh-action-pypi-publish` with GitHub OIDC and `environment: pypi`.

## Notes

- `bgpkit-parser` 0.17.0 requires Rust 1.87.0 (MSRV). We have 1.91.1. ✓
- PyO3 0.28.3 requires Rust 1.83.0. We have 1.91.1. ✓
- The `unsafe impl Send/Sync` for `Parser` must be verified after the bump. If the v0.17.0 `ElemIterator` is no longer `Send`, we use a different approach.
- `maturin-action` builds `manylinux` wheels automatically. No custom `Dockerfile` needed for CI.
- Keep `Dockerfile` and `build.sh` as manual fallbacks. Remove in a future cleanup PR.
- The CI workflow will produce **5 ABI3 wheels** (one per supported platform) + **1 sdist** per release.
