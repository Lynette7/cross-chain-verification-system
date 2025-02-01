const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider });

    api.query.beefy.subscribeSignedCommitments((commitment) => {
        const blockNumber = commitment.blockNumber.toString();
        const validatorSetId = commitment.validatorSetId.toString();
        const merkleRoot = commitment.payload.merkleRoot.toHex();

        console.log(`New BEEFY Commitment:
            Block: ${blockNumber}
            Validator Set ID: ${validatorSetId}
            Merkle Root: ${merkleRoot}
        `);
    });
}

main().catch(console.error);
