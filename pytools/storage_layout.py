import json
from typing import Dict

import solcx


def get_solc_input_json(contract_sources: Dict[str, str]):
    sources_input = {}
    for path, content in contract_sources.items():
        sources_input[path] = {"content": content}
    res = {
        "language": "Solidity",
        "sources": sources_input,
        "settings": {
            "outputSelection": {
                "*": {
                    "*": ["storageLayout"]
                }
            }
        }
    }
    return res


contract_path = '../example/contract.sol'
output_path = '../example/solc_output.json'
solc_version = "v0.8.26"
with open(contract_path) as fp:
    contract_source_content = fp.read()
contract_sources = {
    contract_path: contract_source_content
}
solcx.install_solc(version=solc_version, show_progress=True)
results = solcx.compile_standard(
    input_data=get_solc_input_json(contract_sources),
    solc_version=solc_version,
)
with open(output_path, 'w') as fp:
    json.dump(results["contracts"][contract_path]["MyContract"]["storageLayout"], fp, indent=2)
