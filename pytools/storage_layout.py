import json
from typing import Dict

import solcx


def get_solc_input_json(source_codes: Dict[str, str]):
    sources_input = {}
    for path, content in source_codes.items():
        sources_input[path] = {"content": content}
    res = {
        "language": "Solidity",
        "sources": sources_input,
        "settings": {
            "outputSelection": {
                "*": {
                    "*": "storageLayout"
                }
            }
        }
    }
    return res


contract_path = '../example/contract.sol'
output_path = '../example/solc_output.json'
with open(contract_path) as fp:
    contract_source = fp.read()
result = solcx.compile_source(
    contract_source,
    output_values=["storage-layout"],
    solc_version="v0.8.26",
)
with open(output_path, 'w') as fp:
    json.dump(result["<stdin>:MyContract"]["storage-layout"], fp, indent=2)
