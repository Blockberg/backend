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
  
  
  const [configPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );
  
  console.log("Config PDA:", configPDA.toBase58());
  
  try {
    const config = await program.account.programConfig.fetch(configPDA);
    console.log("\nConfig already initialised!");
    console.log("   Authority:", config.authority.toBase58());
    console.log("   Treasury:", config.treasury.toBase58());
    console.log("   Executors:", config.authorizedExecutors.length);
    console.log("\nNo need to reinitialise.");
    return;
  } catch (e) {
    console.log("\nConfig not yet initialised, continuing...\n");
  }
  
  const treasuryPubkey = provider.wallet.publicKey;
  
  console.log("Treasury (who will receive the fees):", treasuryPubkey.toBase58());
  console.log("\nSending the transaction...\n");
  
  try {
    const tx = await program.methods
      .initializeConfig(treasuryPubkey)
      .accounts({
        authority: provider.wallet.publicKey,
      })
      .rpc();
    
    console.log("Config initialised successfully!");
    console.log("Transaction:", tx);
    console.log("Explorer:", `https://explorer.solana.com/tx/${tx}?cluster=devnet`);
    
    console.log("\nVerification...");
    const config = await program.account.programConfig.fetch(configPDA);
    console.log("\nConfig created:");
    console.log("   ✓ Authority:", config.authority.toBase58());
    console.log("   ✓ Treasury:", config.treasury.toBase58());
    console.log("   ✓ Bump:", config.bump);
    console.log("   ✓ Authorized executors:", config.authorizedExecutors.length);
    
    console.log("\n" + "=".repeat(50));
    console.log("INITIALISATION TERMINÉE !");
    console.log("=".repeat(50));
    console.log("\nNext steps:");
    console.log("   1. Add executors with add_executor()");
    console.log("   2. The users can now create accounts");
    console.log("   3. Start trading!");
    
  } catch (error) {
    console.error("\nError during initialisation:");
    console.error(error);
    throw error;
  }
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