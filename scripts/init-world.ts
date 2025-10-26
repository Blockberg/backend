import * as anchor from '@coral-xyz/anchor';
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { InitializeNewWorld, BN } from '@magicblock-labs/bolt-sdk';
import fs from 'fs';

const MAGICBLOCK_RPC = 'https://rpc.magicblock.app/devnet/';

async function main() {
    console.log('Initializing Bolt World...');

    const connection = new Connection(MAGICBLOCK_RPC, 'confirmed');

    const walletPath = process.env.HOME + '/.config/solana/id.json';
    const secretKey = JSON.parse(fs.readFileSync(walletPath, 'utf-8'));
    const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    console.log('Payer:', payer.publicKey.toBase58());

    const balance = await connection.getBalance(payer.publicKey);
    console.log('Balance:', balance / LAMPORTS_PER_SOL, 'SOL');

    if (balance < 0.1 * LAMPORTS_PER_SOL) {
        console.log('Requesting airdrop...');
        const sig = await connection.requestAirdrop(payer.publicKey, 1 * LAMPORTS_PER_SOL);
        await connection.confirmTransaction(sig);
        console.log('Airdrop confirmed');
    }

    const { transaction, worldPda, worldId } = await InitializeNewWorld({
        payer: payer.publicKey,
        connection,
    });

    console.log('World ID:', worldId.toString());
    console.log('World PDA:', worldPda.toBase58());

    transaction.feePayer = payer.publicKey;
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.sign(payer);

    const signature = await connection.sendRawTransaction(transaction.serialize());
    console.log('Transaction signature:', signature);

    await connection.confirmTransaction(signature);
    console.log('World initialized successfully!');

    const config = {
        worldId: worldId.toString(),
        worldPda: worldPda.toBase58(),
        timestamp: new Date().toISOString(),
    };

    fs.writeFileSync('world-config.json', JSON.stringify(config, null, 2));
    console.log('World config saved to world-config.json');
}

main().catch(console.error);
