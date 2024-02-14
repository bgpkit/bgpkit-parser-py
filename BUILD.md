# Build and Publish Guide

## Pre-requisites

- `maturin`
- `docker`

## Checklist

1. run [`build.sh`](./build.sh) on Apple Silicon Mac
2. run [`build.sh`](./build.sh) inside docker on Apple Silicon Mac
3. run [`build.sh`](./build.sh) on Intel Mac
4. run [`build.sh`](./build.sh) inside docker on Intel Mac

## Build Linux packages in Docker

Build image using the [Dockerfile](./Dockerfile) provided
```
docker build -t bgpkit-builder:latest .
```

Run `docker run --rm -it bgpkit-builder:latest bash` to open a shell in the container
```bash
####
# TODO: copy the content of .pypirc to the root folder
####

bash build.sh
```