import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PaperTrading } from "../target/types/paper_trading";

async function main() {
  // Setup
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.PaperTrading as Program<PaperTrading>;
  
  console.log("=".repeat(50));
  console.log(" INITIALISING THE CONTRACT");
  console.log("=".repeat(50));
  console.log("\nProgram ID:", program.programId.toBase58());
  console.log("Admin wallet:", provider.wallet.publicKey.toBase58());
  
const backendExecutorPubkey = new PublicKey("BACKEND_WALLET_PUBKEY");

console.log("\nAdding executor to whitelist...");

const addTx = await program.methods
  .addExecutor(backendExecutorPubkey)
  .accounts({
    authority: provider.wallet.publicKey,
  })
  .rpc();

console.log("Executor added!");
console.log("Transaction:", addTx);

}

main()
  .then(() => {
    console.log("\nScript terminated successfully");
    process.exit(0);
  })
  .catch((error) => {
    console.error("\nFatal error:");
    console.error(error);
    process.exit(1);
  });