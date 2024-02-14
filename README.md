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

- Python3.7
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

### Publish for Linux

Install multiple Python interpreters:

```bash
sudo apt install software-properties-common
sudo add-apt-repository ppa:deadsnakes/ppa

sudo apt install -y python3.7 python3.7-distutils
sudo apt install -y python3.8 python3.8-distutils
sudo apt install -y python3.9 python3.9-distutils
sudo apt install -y python3.10 python3.10-distutils
sudo apt install -y python3.11 python3.11-distutils
sudo apt install -y python3.12 python3.12-distutils

curl -sS https://bootstrap.pypa.io/get-pip.py | python3.7
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.8
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.9
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.10
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.11
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.12

# install maturin
python3.12 -m pip install maturin patchelf
```

Build and upload for multiple interpreter versions:
```bash
maturin publish --interpreter python3.7 --skip-existing
maturin publish --interpreter python3.8 --skip-existing
maturin publish --interpreter python3.9 --skip-existing
maturin publish --interpreter python3.10 --skip-existing
maturin publish --interpreter python3.11 --skip-existing
maturin publish --interpreter python3.12 --skip-existing
```

#### Using docker

Build image using the [Dockerfile](./Dockerfile) provided
```
docker build -t bgpkit-builder:latest .
docker run --rm -it bgpkit-builder:latest bash
```

Run `docker run --rm -it bgpkit-builder:latest bash` to open a shell in the container
```bash
####
# TODO: copy the content of .pypirc to the root folder
####
git clone https://github.com/bgpkit/bgpkit-parser-py.git
cd bgpkit-parser-py

maturin publish --interpreter python3.7 --skip-existing
maturin publish --interpreter python3.8 --skip-existing
maturin publish --interpreter python3.9 --skip-existing
maturin publish --interpreter python3.10 --skip-existing
maturin publish --interpreter python3.11 --skip-existing
maturin publish --interpreter python3.12 --skip-existing
```

### Publish for MacOS

#### M1-based

**Minimum support version for M1 Macs is Python 3.8**

Install multiple Python interpreters:
```bash
brew install python@3.8
brew install python@3.9
brew install python@3.10
brew install python@3.11
brew install python@3.12
```

```bash
maturin publish --interpreter python3.8 --skip-existing
maturin publish --interpreter python3.9 --skip-existing
maturin publish --interpreter python3.10 --skip-existing
maturin publish --interpreter python3.11 --skip-existing
maturin publish --interpreter python3.12 --skip-existing
```

#### Intel-based

- [ ] add support for packaging for Intel-based Macs 