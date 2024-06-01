import json
import os
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


def generate_storage_layout(contracts_dir: str, output_path: str, solc_version: str):
    contract_sources = {}
    for root, _, files in os.walk(contracts_dir):
        for file in files:
            file_path = os.path.join(root, file)
            with open(file_path) as fp:
                contract_source_content = fp.read()
            contract_reldir = os.path.relpath(root, contracts_dir)
            contract_path = os.path.normpath(os.path.join(contract_reldir, file))
            contract_sources[contract_path] = contract_source_content

    solcx.install_solc(version=solc_version, show_progress=True)
    results = solcx.compile_standard(
        input_data=get_solc_input_json(contract_sources),
        solc_version=solc_version,
    )
    with open(output_path, 'w') as fp:
        json.dump(results, fp, indent=2)


if __name__ == '__main__':
    output_path = '../example/solc_output.json'

    generate_storage_layout("../example/contracts/simple", output_path, "v0.8.26")
    # generate_storage_layout("../example/contracts/uniswap3pool", output_path, "v0.7.6")
