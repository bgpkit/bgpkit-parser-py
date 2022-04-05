from pybgpkit_parser import Parser

parser = Parser(url="https://spaces.bgpkit.org/parser/update-example", filters={"peer_ips": "185.1.8.65, 2001:7f8:73:0:3:fa4:0:1"})

for elem in parser:
    print(elem["origin_asns"])
