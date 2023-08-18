import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import * as bs58 from "bs58";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
} from "@project-serum/anchor";
import { WbaPrereq, IDL } from "./programs/wba_prereq";

// import our keypir from the wallet file
const keypair = Keypair.fromSecretKey(
  bs58.decode(
    "5LRgJ2qGhsWpDiBDSY1BxznQGtRGYcoqYJFTVTdDKoSoXWenhgakQE1mnMPeRMGLZNDFQ5z1tL1t6mKmmWanZ177"
  )
);

// create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// github account
const github = Buffer.from("PrakyathReddy", "utf8");

// create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

// create our program
const program = new Program<WbaPrereq>(
  IDL,
  "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1" as Address,
  provider
);

// create a PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);

// execute our enrollment transaction
(async () => {
  try {
    const txhash = await program.methods
      .complete(github)
      .accounts({
        signer: keypair.publicKey,
        prereq: enrollment_key,
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
