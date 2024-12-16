import subprocess
import json

# Configuración
CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgq5tlkn9qcza52v4dtfzsx99xjxrgtw2p74wzqnyc5ea"
PEM_FILE = "../wallet_shard0.pem"
PROXY = "https://devnet-gateway.multiversx.com"
CHAIN = "D"
WASM_FILE = "output/gass-pass-hack.wasm"

# Función para desplegar el contrato
def deploy_contract():
    deploy_command = [
        "mxpy", "contract", "deploy",
        "--bytecode", WASM_FILE,
        "--pem", PEM_FILE,
        "--gas-limit", "5000000",
        "--chain", CHAIN,
        "--recall-nonce",
        "--proxy", PROXY,
        "--send"
    ]
    result = subprocess.run(deploy_command, capture_output=True, text=True)
    print("Deploy Result:", result.stdout)
    return json.loads(result.stdout).get("contractAddress")

# Función para llamar al contrato
def call_contract(gas_limit):
    call_command = [
        "mxpy", "contract", "call", CONTRACT_ADDRESS,
        "--function", "gaspass",
        "--gas-limit", str(gas_limit),
        "--pem", PEM_FILE,
        "--chain", CHAIN,
        "--recall-nonce",
        "--proxy", PROXY,
        "--send"
    ]
    result = subprocess.run(call_command, capture_output=True, text=True)
    print(f"Result for gas limit {gas_limit}:", result.stdout)
    return result.stdout

# Despliega el contrato y prueba diferentes valores de gas
if __name__ == "__main__":
    contract_address = deploy_contract()
    for gas in range(3000000, 3100000, 1000):
        output = call_contract(gas)
        if "true" in output:
            print(f"¡Gas encontrado!: {gas}")
            break
