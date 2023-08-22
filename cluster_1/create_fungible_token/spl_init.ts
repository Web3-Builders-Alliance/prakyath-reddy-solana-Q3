import { createMint } from "@solana/spl-token";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

async function createTokenMint(secretKeyString: string) {
  // Decode secret key from base58 format and create a keypair
  const secretKeyUint8Array = bs58.decode(secretKeyString);
  const walletKeypair = Keypair.fromSecretKey(secretKeyUint8Array);

  const payer = walletKeypair; // Using my WBA wallet as the payer
  const mintAuthority = walletKeypair; // Using my WBA wallet as the mint authority

  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const mint = await createMint(
    connection,
    payer,
    mintAuthority.publicKey,
    null,
    6 // Decimals
  );

  console.log(mint.toBase58());
}

// Enter wallet's secret key in base58 format
const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;

// Call the asynchronous function
createTokenMint(MY_WALLET_SECRET_KEY).catch((error) => {
  console.error("Failed to create token mint:", error);
});
