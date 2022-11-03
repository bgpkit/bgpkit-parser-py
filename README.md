# bgpkit-parser-py

Python binding for bgpkit-parser

## Supported Python Version

- Python3.6
- Python3.7
- Python3.8
- Python3.9
- Python3.10
- Python3.11

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
maturin publish --interpreter python3.10 --skip-existing
maturin publish --interpreter python3.11 --skip-existing
```

### Publish for MacOS

```bash
maturin publish --skip-existing
```
