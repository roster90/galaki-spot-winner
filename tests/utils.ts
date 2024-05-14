import * as anchor from "@coral-xyz/anchor";

import { Connection, Keypair, LAMPORTS_PER_SOL,  PublicKey, SOLANA_SCHEMA, Signer, SystemProgram } from "@solana/web3.js";
import { Program, AnchorProvider, web3, utils, BN } from '@project-serum/anchor';
import { assert, expect } from "chai";


  const ADMIN_ROLE =  "ADMIN_ROLE";
  const OPERATOR_ROLE =  "OPERATOR_ROLE";

  const PLAYER_ACCOUNT =  "PLAYER";

  const GAME_PROJECT =  "GAME_PROJECT";

  const GALAKI_WINNER =  "GALAKI_WINNER";

export const getGalakiPda = (programId:  PublicKey) => {
    const [mint, _] = PublicKey.findProgramAddressSync([Buffer.from(GALAKI_WINNER)], programId);
    console.log("Galaki PDA", mint.toString());
    return mint;
}

export const getGameProjectPda = (programId:  PublicKey, game_id : anchor.BN) => {
    const [mint, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(GAME_PROJECT),
            game_id.toBuffer("le", 8),
        ]
        , programId);
    console.log("Project PDA", mint.toString());
    return mint;
}

export const getAdminRolePda = (programId:  PublicKey, user: PublicKey) => {
    const [mint, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(ADMIN_ROLE),
            user.toBuffer(),
        ]
        , programId);
    console.log("Admin PDA", mint.toString());
    return mint;

}

export const getOperatorRolePda = (programId:  PublicKey, user: PublicKey) => {
    const [mint, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(OPERATOR_ROLE),
            user.toBuffer(),
        ]
        , programId);
    console.log("operator PDA", mint.toString());
    return mint;
}

export const getPlayerAccountPda = (programId:  PublicKey, game_id: BN, user: PublicKey) => {
    const [mint, _] = PublicKey.findProgramAddressSync(
        [Buffer.from(PLAYER_ACCOUNT),
            game_id.toBuffer("le", 8),
            user.toBuffer(),
        ]
        , programId);
    console.log("player account PDA", mint.toString());
    return mint;

}