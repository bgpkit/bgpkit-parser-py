FROM ubuntu:jammy

RUN apt update && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt install -y curl libssl-dev pkg-config build-essential software-properties-common tzdata git vim cmake

# install different versions of Python
RUN add-apt-repository ppa:deadsnakes/ppa -y && apt update
RUN apt install -y \
python3.7 python3.7-distutils \
python3.8 python3.8-distutils \
python3.9 python3.9-distutils \
python3.10 python3.10-distutils \
python3.11 python3.11-distutils \
python3.12 python3.12-distutils

RUN \
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.8 && \
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.9 && \
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.10 && \
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.11 && \
curl -sS https://bootstrap.pypa.io/get-pip.py | python3.12

# install maturin
RUN python3.12 -m pip install maturin patchelf
# install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /io/bgpkit-parser-py
COPY ./src ./src
COPY ./build.rs .
COPY ./build.sh .
COPY ./Cargo.toml .
COPY ./README.md .
COPY ./pyproject.toml .
