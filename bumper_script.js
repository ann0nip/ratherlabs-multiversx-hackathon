const fs = require('fs');
const {
    Address,
    Account,
    Transaction,
    TransactionPayload,
} = require('@multiversx/sdk-core');
const { ProxyNetworkProvider } = require('@multiversx/sdk-network-providers');
const { UserSigner } = require('@multiversx/sdk-wallet');

async function main() {
    const gateway = 'https://devnet-gateway.multiversx.com';
    const bumpAddress = new Address(
        'erd1qqqqqqqqqqqqqpgq23j27f6w0r75hfyc5td753f9ahvfpp5x4wzq65czqw'
    );

    const pem = fs.readFileSync('./wallet.pem', { encoding: 'utf8' });
    const signer = UserSigner.fromPem(pem);

    const provider = new ProxyNetworkProvider(gateway);
    const networkConfig = await provider.getNetworkConfig();
    const chainID = networkConfig.ChainID;

    // Esta dirección ya es un objeto Address válido
    const address = await signer.getAddress();

    // Obt取ner el nonce inicial y actualizarlo dinámicamente
    let currentNonce = (await provider.getAccount(address)).nonce.valueOf();

    const numberOfBumps = 1000000000;

    for (let i = 0; i < numberOfBumps; i++) {
        try {
            // Actualizar el nonce antes de cada transacción
            const accountOnNetwork = await provider.getAccount(address);
            currentNonce = accountOnNetwork.nonce.valueOf();

            const data = new TransactionPayload('bump');

            const tx = new Transaction({
                nonce: currentNonce,
                value: '0',
                data,
                receiver: bumpAddress,
                gasLimit: 2000000,
                sender: address,
                chainID: chainID,
            });

            const serializedTransaction = tx.serializeForSigning();
            const signature = await signer.sign(serializedTransaction);
            tx.signature = signature;

            const txHash = await provider.sendTransaction(tx);
            console.log(`Bump #${i + 1} sent, txHash:`, txHash);
        } catch (error) {
            console.error(`Error en bump #${i + 1}:`, error.message);

            // Esperar un poco antes de continuar
            await new Promise((resolve) => setTimeout(resolve, 1000));
        }
    }

    console.log(`Enviadas ${numberOfBumps} transacciones 'bump' exitosamente!`);
}

main().catch(console.error);
