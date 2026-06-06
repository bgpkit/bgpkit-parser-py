# Build and Publish Guide

## Automated Release (Recommended)

Release builds are handled by GitHub Actions via `.github/workflows/release.yml`.

Push a version tag to build and publish:

```bash
git tag v0.7.0
git push origin v0.7.0
```

The release workflow will:

1. Run `cargo fmt --check` and `cargo clippy -- -D warnings`
2. Build the source distribution (`sdist`)
3. Build ABI3 wheels for:
   - Linux x86_64
   - Linux aarch64
   - macOS x86_64
   - macOS arm64
   - Windows x86_64
4. Publish artifacts to PyPI using `PYPI_API_TOKEN`
5. Create a GitHub Release and attach the built artifacts

Manual workflow runs (`workflow_dispatch`) are build-only by default. They only publish when the `publish` input is explicitly enabled.

## Required GitHub Secret

Create a project-scoped PyPI API token and save it as a GitHub Actions secret:

```text
PYPI_API_TOKEN
```

Recommended token scope: only the `pybgpkit-parser` project.

## Local Development Build

```bash
maturin develop
```

This builds the extension and installs it into the active Python environment.

## Local Wheel Build

```bash
maturin build --release
```

Built wheels are written under `target/wheels/`.

## Manual Publish Fallback

If CI is unavailable, build locally and upload with `twine`:

```bash
python -m pip install --upgrade maturin twine
maturin build --release --sdist
twine upload --skip-existing target/wheels/*
```

The historical `build.sh` and `Dockerfile` are kept as fallback tools for reproducing older manual builds, but CI is the preferred release path.
