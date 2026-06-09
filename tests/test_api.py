import os
import pytest

from pybgpkit_parser import (
    ELEM_TYPE_ANNOUNCE,
    ELEM_TYPE_WITHDRAW,
    BASIC_FIELDS,
    NEXT_HOP_FIELDS,
    ROUTE_FIELDS,
    Filter,
    PSV_HEADER,
    Parser,
    RouteParser,
)

URL = "https://spaces.bgpkit.org/parser/update-example"


def test_filter_repr_and_helpers():
    filt = Filter("peer_ips", "185.1.8.65,2001:7f8:73:0:3:fa4:0:1")
    assert "Filter" in repr(filt)
    assert "Filter" in repr(Filter.peer_ip("185.1.8.65"))
    assert "Filter" in repr(Filter.peer_ips(["185.1.8.65", "2001:7f8:73:0:3:fa4:0:1"]))
    assert "Filter" in repr(Filter.origin_asn(13335))
    assert "Filter" in repr(Filter.prefix("1.1.1.0/24"))
    assert "Filter" in repr(Filter.elem_type("a"))


def test_module_constants():
    assert ELEM_TYPE_ANNOUNCE == "A"
    assert ELEM_TYPE_WITHDRAW == "W"
    assert PSV_HEADER.startswith("type|timestamp")
    assert BASIC_FIELDS == ["timestamp", "elem_type", "peer_ip", "peer_asn", "prefix"]
    assert ROUTE_FIELDS[-1] == "as_path"
    assert NEXT_HOP_FIELDS[-1] == "next_hop"


def test_invalid_filter_raises_value_error():
    with pytest.raises(ValueError):
        Filter("peer_ips", "not-an-ip")


@pytest.mark.skipif(
    os.environ.get("PYBGPKIT_RUN_NETWORK_TESTS") != "1",
    reason="network smoke test; set PYBGPKIT_RUN_NETWORK_TESTS=1 to enable",
)
def test_parser_iteration_and_elem_api_network():
    parser = Parser.from_filters(URL, [Filter("peer_ips", "185.1.8.65")])
    elem = parser.parse_next()
    assert elem is not None

    data = elem.to_dict()
    assert elem.as_dict() == data
    assert "peer_bgp_id" in data
    assert "only_to_customer" in data
    assert elem.elem_type in {"A", "W"}
    assert elem.is_announcement() or elem.is_withdrawal()
    assert elem.has_as_path() == (elem.as_path is not None)
    assert elem.origin_asn == elem.get_origin_asn()
    assert isinstance(elem.to_json(), str)
    assert isinstance(elem.to_psv(), str)
    assert "Elem" in repr(elem)


def test_parser_count_and_batches_network():
    if os.environ.get("PYBGPKIT_RUN_NETWORK_TESTS") != "1":
        pytest.skip("network smoke test; set PYBGPKIT_RUN_NETWORK_TESTS=1 to enable")

    parser = Parser(URL, filters={"peer_ips": "185.1.8.65"})
    assert parser.count() > 0
    assert parser.count() == 0

    parser = Parser(URL, filters={"peer_ips": "185.1.8.65"})
    batches = parser.iter_batches(1000)
    first = next(batches)
    assert first
    assert len(first) <= 1000

    parser = Parser(URL, filters={"peer_ips": "185.1.8.65"})
    row = next(parser.iter_tuples(["peer_ip", "peer_asn", "prefix"]))
    assert len(row) == 3

    parser = Parser(URL, filters={"peer_ips": "185.1.8.65"})
    batch = next(parser.iter_tuple_batches(["peer_ip", "prefix"], 1000))
    assert batch
    assert len(batch[0]) == 2


def test_route_parser_network():
    if os.environ.get("PYBGPKIT_RUN_NETWORK_TESTS") != "1":
        pytest.skip("network smoke test; set PYBGPKIT_RUN_NETWORK_TESTS=1 to enable")

    parser = RouteParser.from_filters(URL, [Filter.peer_ip("185.1.8.65")])
    route = parser.parse_next()
    assert route is not None
    assert route.elem_type in {"A", "W"}
    assert route.as_dict() == route.to_dict()
    assert route.has_as_path() == (route.as_path is not None)
    assert "RouteElem" in repr(route)

    parser = RouteParser(URL, filters={"peer_ips": "185.1.8.65"})
    assert parser.count() > 0

    parser = RouteParser(URL, filters={"peer_ips": "185.1.8.65"})
    first = next(parser.iter_batches(1000))
    assert first
    assert len(first) <= 1000

    parser = RouteParser(URL, filters={"peer_ips": "185.1.8.65"})
    row = next(parser.iter_tuples(["peer_ip", "peer_asn", "prefix"]))
    assert len(row) == 3

    parser = RouteParser(URL, filters={"peer_ips": "185.1.8.65"})
    batch = next(parser.iter_tuple_batches(["peer_ip", "prefix"], 1000))
    assert batch
    assert len(batch[0]) == 2
