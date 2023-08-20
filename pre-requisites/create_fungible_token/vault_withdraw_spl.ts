import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import * as bs58 from "bs58";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@project-serum/anchor";
import { wba_vault, IDL } from "./wba_vault";
import dotenv from "dotenv";
import { Key } from "@metaplex-foundation/mpl-token-metadata";
import {
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
dotenv.config();

// declare our wallet secret key
const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;
const secretKeyUint8Array = bs58.decode(MY_WALLET_SECRET_KEY);
const keypair = Keypair.fromSecretKey(secretKeyUint8Array);

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Github account
const github = Buffer.from("PrakyathReddy", "utf8");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

// Create our program
const program = new Program<wba_vault>(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider
);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);
const vaultState = new PublicKey(
  "8oJB65i8gDMwMWTgk2mozdkeF2bMFQLSv6LcG4npAU5A"
);
const vaultAuth = PublicKey.findProgramAddressSync(
  [Buffer.from("auth"), vaultState.toBuffer()],
  program.programId
)[0];
const vault = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), vaultAuth.toBuffer()],
  program.programId
)[0];
const mint = new PublicKey("4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN");

// Execute our enrollment transaction
(async () => {
  try {
    const owner_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    const vault_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      vaultAuth,
      true
    );

    const txhash = await program.methods
      .withdrawSpl(new BN(1e6))
      .accounts({
        owner: keypair.publicKey,
        ownerAta: owner_ata.address,
        vaultAta: vault_ata.address,
        tokenMint: mint,
        vaultState: vaultState,
        vaultAuth: vaultAuth,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
