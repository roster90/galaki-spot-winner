import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GalakiWinnerSpotSc } from "../target/types/galaki_winner_spot_sc";
import { getAdminRolePda, getGalakiPda } from "./utils";
import {
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
describe("galaki-winner-spot-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GalakiWinnerSpotSc as Program<GalakiWinnerSpotSc>;

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const galaki_pda = getGalakiPda(program.programId);
  const getAdminPda = getAdminRolePda(program.programId, provider.wallet.publicKey);
  const operator_wallet = new PublicKey("9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS")
  // it("Is initialized galaki!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize(operator_wallet).accounts({
  //     galakiAccount: galaki_pda,
  //     adminAccount: getAdminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();
  //   console.log("Your transaction signature", tx);

  //   const state = await program.account.galaki.fetch(galaki_pda);
  //   console.log("State", JSON.stringify(state));



  // });

  it("Test random", async () => {
    try {
       // Add your test here.
    const tx = await program.methods.requestRandomness().accounts({
      payer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      // slotHashes: anchor.web3.SYSVAR_SLOT_HASHES_PUBKEY,
    }).rpc();
    console.log("Your transaction signature", tx);

 
    } catch (error) {
        console.log("Error", error);
        
    }
   

    

  });
});
