import * as anchor from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { BN } from '@magicblock-labs/bolt-sdk';
import fs from 'fs';

const MAGICBLOCK_RPC = 'https://rpc.magicblock.app/devnet/';

const COMPONENT_IDS = {
    TRADING_ACCOUNT: new PublicKey('3UhnNbUpRi1QM6szPYJce4tBNLCbjxMESJJ8touBd55h'),
    COMPETITION: new PublicKey('zQKpawEnbpdRj7MPzPuBKjJdgmSCC2A1aNi3NbGv4PN'),
    POSITION: new PublicKey('8NHfJVx1ZD8tnb23v4xvTsUdhMxhHbjYpPz4ZDstobYP'),
    LEADERBOARD: new PublicKey('5ohgmFUcN41uoZuP1QnFP9ErjDDCXA1FaxpFZAzfwU6q'),
};

async function main() {
    console.log('Registering components with World...');

    const worldConfig = JSON.parse(fs.readFileSync('world-config.json', 'utf-8'));
    const WORLD_PDA = new PublicKey(worldConfig.worldPda);

    console.log('World PDA:', WORLD_PDA.toBase58());

    const connection = new Connection(MAGICBLOCK_RPC, 'confirmed');

    const walletPath = process.env.HOME + '/.config/solana/id.json';
    const secretKey = JSON.parse(fs.readFileSync(walletPath, 'utf-8'));
    const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    console.log('Payer:', payer.publicKey.toBase58());

    const worldProgramId = new PublicKey('WorLD15A7CrDwLcLy4fRqtaTb9fbd8o8iqiEMUDse2n');

    const provider = new anchor.AnchorProvider(
        connection,
        new anchor.Wallet(payer),
        { commitment: 'confirmed' }
    );

    const idl = {
        version: "0.1.0",
        name: "world",
        instructions: [
            {
                name: "registerComponent",
                accounts: [
                    { name: "world", isMut: true, isSigner: false },
                    { name: "authority", isMut: false, isSigner: true },
                    { name: "componentProgram", isMut: false, isSigner: false },
                ],
                args: [],
            }
        ]
    };

    const program = new anchor.Program(idl as any, provider);

    for (const [name, componentId] of Object.entries(COMPONENT_IDS)) {
        try {
            console.log(`\nRegistering ${name}...`);

            const tx = await program.methods
                .registerComponent()
                .accounts({
                    world: WORLD_PDA,
                    authority: payer.publicKey,
                    componentProgram: componentId,
                })
                .rpc();

            console.log(`${name} registered:`, tx);
        } catch (e: any) {
            console.error(`Failed to register ${name}:`, e.message);
        }
    }

    console.log('\nComponent registration complete!');
}

main().catch(console.error);
