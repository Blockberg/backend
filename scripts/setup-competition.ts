import * as anchor from '@coral-xyz/anchor';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import {
    AddEntity,
    InitializeComponent,
    ApproveSystem,
    FindEntityPda,
    FindComponentPda,
    BN
} from '@magicblock-labs/bolt-sdk';
import fs from 'fs';

const MAGICBLOCK_RPC = 'https://rpc.magicblock.app/devnet/';

const COMPONENT_IDS = {
    COMPETITION: new PublicKey('zQKpawEnbpdRj7MPzPuBKjJdgmSCC2A1aNi3NbGv4PN'),
};

const SYSTEM_IDS = {
    JOIN_COMPETITION: new PublicKey('FFRL7nSQxFYMEcUxb912WsvbMSDMPNefdgCe4aZYNxWk'),
    OPEN_POSITION: new PublicKey('B1LMnYAtxvQLFG56YS9vscBFLid7KHp1nWTPYtFKFLPh'),
    CLOSE_POSITION: new PublicKey('49kLMtwwnm5wdCKvtToUYgoxxo9PXjheS1rwcoSYkQfG'),
    SETTLE_COMPETITION: new PublicKey('C1FTdtq531t4MViYtgo7LAft3GRkJimYAhVWFU4BE46i'),
};

async function main() {
    console.log('Setting up Competition...');

    const worldConfig = JSON.parse(fs.readFileSync('world-config.json', 'utf-8'));
    const WORLD_ID = new BN(worldConfig.worldId);
    const WORLD_PDA = new PublicKey(worldConfig.worldPda);

    console.log('World ID:', WORLD_ID.toString());
    console.log('World PDA:', WORLD_PDA.toBase58());

    const connection = new Connection(MAGICBLOCK_RPC, 'confirmed');

    const walletPath = process.env.HOME + '/.config/solana/id.json';
    const secretKey = JSON.parse(fs.readFileSync(walletPath, 'utf-8'));
    const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    console.log('Payer:', payer.publicKey.toBase58());

    console.log('\n1. Creating competition entity...');
    const { transaction: addEntityTx, entityPda: competitionEntityPda } = await AddEntity({
        payer: payer.publicKey,
        world: WORLD_PDA,
        seed: Buffer.from('competition'),
        connection,
    });

    console.log('Competition Entity PDA:', competitionEntityPda.toBase58());

    addEntityTx.feePayer = payer.publicKey;
    let blockhash = (await connection.getLatestBlockhash()).blockhash;
    addEntityTx.recentBlockhash = blockhash;
    addEntityTx.sign(payer);

    let sig = await connection.sendRawTransaction(addEntityTx.serialize());
    await connection.confirmTransaction(sig);
    console.log('Entity created:', sig);

    await new Promise(resolve => setTimeout(resolve, 2000));

    console.log('\n2. Initializing competition component...');
    const { transaction: initCompTx } = await InitializeComponent({
        payer: payer.publicKey,
        entity: competitionEntityPda,
        componentId: COMPONENT_IDS.COMPETITION,
    });

    initCompTx.feePayer = payer.publicKey;
    blockhash = (await connection.getLatestBlockhash()).blockhash;
    initCompTx.recentBlockhash = blockhash;
    initCompTx.sign(payer);

    sig = await connection.sendRawTransaction(initCompTx.serialize());
    await connection.confirmTransaction(sig);
    console.log('Competition component initialized:', sig);

    console.log('\n3. Approving systems...');
    for (const [name, systemId] of Object.entries(SYSTEM_IDS)) {
        console.log(`Approving ${name}...`);
        const { transaction: approveTx } = await ApproveSystem({
            authority: payer.publicKey,
            systemToApprove: systemId,
            world: WORLD_PDA,
        });

        approveTx.feePayer = payer.publicKey;
        blockhash = (await connection.getLatestBlockhash()).blockhash;
        approveTx.recentBlockhash = blockhash;
        approveTx.sign(payer);

        sig = await connection.sendRawTransaction(approveTx.serialize());
        await connection.confirmTransaction(sig);
        console.log(`${name} approved:`, sig);
    }

    const config = {
        ...worldConfig,
        competitionEntity: competitionEntityPda.toBase58(),
        systemsApproved: Object.keys(SYSTEM_IDS),
    };

    fs.writeFileSync('world-config.json', JSON.stringify(config, null, 2));
    console.log('\nSetup complete! Config saved to world-config.json');
}

main().catch(console.error);
