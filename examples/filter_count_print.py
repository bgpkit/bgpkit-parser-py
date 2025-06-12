from pybgpkit_parser import Parser
import json

parser = Parser(
    url="https://spaces.bgpkit.org/parser/update-example",
    filters={"peer_ips": "185.1.8.65, 2001:7f8:73:0:3:fa4:0:1"},
    cache_dir="./cache"
)

for elem in parser:
    # Directly access the fields of the parsed BGP update
    print(elem.origin_asns)
    print(elem.as_path)

    # Optional fields can be checked for None
    print(elem.aggr_ip is None)

    # Print the entire parsed BGP update as a dictionary
    print(json.dumps(elem.to_dict()))
    print(elem)
    break
