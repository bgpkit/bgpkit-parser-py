# PROJECT KNOWLEDGE BASE

## OVERVIEW

Python binding for `bgpkit-parser` (Rust MRT/BGP parser). Exposes `Parser` (full elems), `RouteParser` (route-level scans), `Filter` helpers, and projected tuple iteration via PyO3, built with maturin.

## STRUCTURE

```
├── src/lib.rs          # Entire Python extension: Parser/elem/route PyO3 classes
├── examples/           # Python usage examples
├── Cargo.toml          # Rust crate: pybgpkit-parser, depends on bgpkit-parser
├── pyproject.toml      # Maturin build-system config
├── build.rs            # PyO3 extension module linker setup
├── benches/            # Rust criterion benchmarks
├── tests/              # Python API tests and benchmark
└── .github/workflows/  # Rust fmt/clippy CI + tag-based release
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Change exposed Python API | `src/lib.rs` |
| Update underlying parser logic | `Cargo.toml` → bump `bgpkit-parser` version |
| Build/test locally | `maturin develop` (see README.md) |
| Build wheels for release | GitHub Actions `release.yml` (push `v*` tag) |
| Publish to PyPI | Push `v*` tag; CI publishes via PyPI Trusted Publishing (OIDC) |

## CODE MAP

- **`Elem`** — PyO3 class wrapping a parsed BGP element. Has `#[pyo3(get, set)]` fields and `to_dict()` / `__str__` / `__getstate__` methods.
- **`Parser`** — PyO3 class wrapping `bgpkit_parser::BgpkitParser`. Constructor takes `url`, optional `filters` (HashMap), and optional `cache_dir`. Implements `__iter__`/`__next__`, `count`, `iter_batches`, `iter_tuples`, and `iter_tuple_batches`.
- **`RouteParser`** — PyO3 class wrapping `BgpkitParser::into_route_iter()`. Returns lightweight `RouteElem` values. Same iteration/helper surface as `Parser`.
- **`Filter`** — PyO3 class wrapping `bgpkit_parser::parser::Filter`. Constructors: `__init__`, `peer_ip`, `peer_ips`, `origin_asn`, `prefix`, `elem_type`.
- **`TupleIterator` / `TupleBatchIterator`** — High-performance projected tuple iteration for `Parser` and `RouteParser`.
- **`convert_elem`** — Internal fn mapping `BgpElem` → `Elem` (Rust type → PyO3 type).

## CONVENTIONS

- Rust fmt/clippy enforced in CI (`cargo fmt --check`, `cargo clippy -- -D warnings`)
- `PyValueError` used for filter errors propagated to Python
- Iterator-backed pyclasses use `#[pyclass(unsendable)]`; no `unsafe impl Send/Sync`
- `#[pyo3(name = "__str__")]` used for JSON string representation of `Elem`
- `atomic` field returns `"AG"`/`"NAG"` strings (not bool)
- `elem_type` field returns `"A"` (announce) or `"W"` (withdraw)

## ANTI-PATTERNS

- **Do NOT** change PyO3/maturin versions without updating both `Cargo.toml` and `build.rs` (`pyo3-build-config` must match)
- **Do NOT** test release publishing with a beta tag unless the package version is also beta; use `workflow_dispatch` with `publish=false` for build-only checks
- **Do NOT** add long-lived PyPI API tokens; use PyPI Trusted Publishing with GitHub OIDC (`environment: pypi`)
- **Do NOT** add `unsafe impl Send/Sync` to `#[pyclass]` types; use `#[pyclass(unsendable)]` instead
- **Do NOT** use `.unwrap()` on user inputs (URL/filters); already handled in `BgpkitParser::new` but be careful with new additions
- **Do NOT** make `Elem` fields write-only or remove getters without noting in CHANGELOG as breaking (v0.6.0 was a breaking change)

## COMMANDS

```bash
# Local dev build (installs to active venv)
maturin develop

# Build wheel locally
maturin build --release

# Build and publish release via CI
git tag v0.7.0
git push origin v0.7.0

# Format + lint
cargo fmt --check
cargo clippy -- -D warnings

# Publish (after building on all platforms)
twine upload --skip-existing target/wheels/*
```

## NOTES

- `bgpkit-parser` crate version bump is the primary release trigger (see CHANGELOG for version history)
- Release workflow: `rust.yaml` runs Rust + Python API checks on PR/push; `release.yml` builds ABI3 wheels and publishes on `v*` tag push via Trusted Publishing
- Supports Python 3.9+ via ABI3 wheels
- Python API tests live in `tests/test_api.py`; network smoke coverage is gated by `PYBGPKIT_RUN_NETWORK_TESTS=1`
