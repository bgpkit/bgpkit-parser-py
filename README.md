# bgpkit-parser-py

Python binding for bgpkit-parser

## Installation

```bash
python3 -m pip install pybgpkit-parser
```

## Develop

`maturin develop` builds local python module and add to the venv.

### Publish for Linux

Install multiple Python interpreters:

```bash
sudo apt install software-properties-common
sudo add-apt-repository ppa:deadsnakes/ppa
```

Build and upload for multiple interpreter versions:
```bash
maturin publish --interpreter python3.6 --skip-existing
maturin publish --interpreter python3.7 --skip-existing
maturin publish --interpreter python3.8 --skip-existing
maturin publish --interpreter python3.9 --skip-existing
```

### Publish for MacOS

```bash
maturin publish --skip-existing
```
