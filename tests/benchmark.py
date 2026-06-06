"""Python-side performance benchmark for pybgpkit-parser.

Run after a release build for meaningful numbers:

    maturin develop --release
    python tests/benchmark.py [path-or-url]

This script intentionally uses a remote BGPKIT sample file. It is meant for
manual performance checks, not deterministic CI assertions.
"""

import os
import sys
import time

from pybgpkit_parser import Parser, RouteParser

URL = sys.argv[1] if len(sys.argv) > 1 else os.environ.get(
    "PYBGPKIT_BENCH_INPUT",
    "https://spaces.bgpkit.org/parser/update-example",
)


def bench_iteration():
    parser = Parser(URL)
    start = time.perf_counter()
    count = 0
    for _elem in parser:
        count += 1
    elapsed = time.perf_counter() - start
    return "iteration", count, elapsed


def bench_parse_all():
    parser = Parser(URL)
    start = time.perf_counter()
    elems = parser.parse_all()
    elapsed = time.perf_counter() - start
    return "parse_all", len(elems), elapsed


def bench_iter_batches(batch_size=1000):
    parser = Parser(URL)
    start = time.perf_counter()
    count = 0
    for batch in parser.iter_batches(batch_size):
        count += len(batch)
    elapsed = time.perf_counter() - start
    return f"iter_batches({batch_size})", count, elapsed


def bench_iter_tuples():
    parser = Parser(URL)
    fields = ["timestamp", "prefix", "as_path"]
    start = time.perf_counter()
    count = 0
    for _row in parser.iter_tuples(fields):
        count += 1
    elapsed = time.perf_counter() - start
    return "iter_tuples(timestamp,prefix,as_path)", count, elapsed


def bench_iter_tuple_batches(batch_size=1000):
    parser = Parser(URL)
    fields = ["timestamp", "prefix", "as_path"]
    start = time.perf_counter()
    count = 0
    for batch in parser.iter_tuple_batches(fields, batch_size):
        count += len(batch)
    elapsed = time.perf_counter() - start
    return f"iter_tuple_batches({batch_size})", count, elapsed


def bench_route_iteration():
    parser = RouteParser(URL)
    start = time.perf_counter()
    count = 0
    for _route in parser:
        count += 1
    elapsed = time.perf_counter() - start
    return "route_iteration", count, elapsed


def bench_route_iter_batches(batch_size=1000):
    parser = RouteParser(URL)
    start = time.perf_counter()
    count = 0
    for batch in parser.iter_batches(batch_size):
        count += len(batch)
    elapsed = time.perf_counter() - start
    return f"route_iter_batches({batch_size})", count, elapsed


def bench_route_iter_tuples():
    parser = RouteParser(URL)
    fields = ["timestamp", "prefix", "as_path"]
    start = time.perf_counter()
    count = 0
    for _row in parser.iter_tuples(fields):
        count += 1
    elapsed = time.perf_counter() - start
    return "route_iter_tuples(timestamp,prefix,as_path)", count, elapsed


def bench_route_iter_tuple_batches(batch_size=1000):
    parser = RouteParser(URL)
    fields = ["timestamp", "prefix", "as_path"]
    start = time.perf_counter()
    count = 0
    for batch in parser.iter_tuple_batches(fields, batch_size):
        count += len(batch)
    elapsed = time.perf_counter() - start
    return f"route_iter_tuple_batches({batch_size})", count, elapsed


def bench_route_parse_all():
    parser = RouteParser(URL)
    start = time.perf_counter()
    routes = parser.parse_all()
    elapsed = time.perf_counter() - start
    return "route_parse_all", len(routes), elapsed


def bench_to_dict():
    parser = Parser(URL)
    elems = parser.parse_all()
    start = time.perf_counter()
    for elem in elems:
        elem.to_dict()
    elapsed = time.perf_counter() - start
    return "to_dict", len(elems), elapsed


def main():
    for name, count, elapsed in [
        bench_iteration(),
        bench_iter_batches(),
        bench_parse_all(),
        bench_iter_tuples(),
        bench_iter_tuple_batches(),
        bench_route_iteration(),
        bench_route_iter_batches(),
        bench_route_parse_all(),
        bench_route_iter_tuples(),
        bench_route_iter_tuple_batches(),
        bench_to_dict(),
    ]:
        rate = count / elapsed if elapsed else 0
        print(f"{name}: {count:,} elems in {elapsed:.3f}s ({rate:,.0f} elems/s)")


if __name__ == "__main__":
    main()
