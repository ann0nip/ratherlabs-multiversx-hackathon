import subprocess
import time
import sys

def run_hack_coinflip():
    """Ejecuta el hack del coinflip"""
    command = """mxpy contract call erd1qqqqqqqqqqqqqpgqst36exda9q777emp52lkk0265zxzkkpxhm2qxqkhs5 \
    --function="hack_coinflip" \
    --recall-nonce \
    --gas-limit=10000000 \
    --pem="wallet_shard0.pem" \
    --chain=D \
    --proxy=https://devnet-gateway.multiversx.com \
    --send"""

    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True)
        print("Hack Coinflip Output:")
        print(result.stdout)
        return result.returncode == 0
    except Exception as e:
        print(f"Error ejecutando hack_coinflip: {e}")
        return False

def donate_after_win():
    """Dona los bumps después de ganar"""
    command = """mxpy contract call erd1qqqqqqqqqqqqqpgqst36exda9q777emp52lkk0265zxzkkpxhm2qxqkhs5 \
    --function="donate_after_win" \
    --recall-nonce \
    --gas-limit=10000000 \
    --pem="wallet_shard0.pem" \
    --chain=D \
    --proxy=https://devnet-gateway.multiversx.com \
    --send"""

    try:
        result = subprocess.run(command, shell=True, capture_output=True, text=True)
        print("Donate After Win Output:")
        print(result.stdout)
        return result.returncode == 0
    except Exception as e:
        print(f"Error ejecutando donate_after_win: {e}")
        return False

def main():
    while True:
        # Ejecuta el hack del coinflip
        print("Ejecutando hack_coinflip...")
        hack_result = run_hack_coinflip()

        # Espera 6 segundos para que la transacción se procese
        print("Esperando 8 segundos...")
        time.sleep(8)

        # Decide si hacer la donación
        if hack_result:
            print("Intentando donación...")
            donate_after_win()

        # Espera antes del próximo intento
        print("Esperando 8 segundos antes del próximo intento...")
        time.sleep(8)

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nScript detenido por el usuario.")
        sys.exit(0)