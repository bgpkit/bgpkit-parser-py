FROM ubuntu:noble

RUN apt update && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt install -y curl libssl-dev pkg-config build-essential software-properties-common tzdata git vim cmake

# install different versions of Python
RUN add-apt-repository ppa:deadsnakes/ppa -y && apt update

RUN apt install -y python3.9 python3.9-distutils
RUN apt install -y python3.10 python3.10-distutils
RUN apt install -y python3.11 python3.11-distutils
RUN apt install -y python3.12
RUN apt install -y python3.13

RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3.9
RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3.10
RUN curl -sS https://bootstrap.pypa.io/get-pip.py | python3.11
RUN curl -sS https://bootstrap.pypa.io/get-pip.py -o get-pip.py && python3.12 get-pip.py --break-system-packages && rm get-pip.py
RUN curl -sS https://bootstrap.pypa.io/get-pip.py -o get-pip.py && python3.13 get-pip.py --break-system-packages && rm get-pip.py

# install maturin
RUN python3.13 -m pip install maturin patchelf twine
# install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /io/bgpkit-parser-py
COPY ./src ./src
COPY ./build.rs .
COPY ./build.sh .
COPY ./Cargo.toml .
COPY ./README.md .
COPY ./pyproject.toml .

COPY ./.pypirc /root/.pypirc