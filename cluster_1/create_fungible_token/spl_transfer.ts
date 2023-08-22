import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import {
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import * as bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

// declare our wallet secret key
const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;
const secretKeyUint8Array = bs58.decode(MY_WALLET_SECRET_KEY);
const keypair = Keypair.fromSecretKey(secretKeyUint8Array);

// create a solana devnet connection
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// mint address
const mint = new PublicKey("4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN");

// recipient address - one of our cadets on WBA
const to = new PublicKey("BvhV49WPYBbzPu8Fpy8YnPnwhNWLbm9Vmdj2T5bNSotS");

(async () => {
  try {
    // getting the token account of the fromWallet address, and if it does not exist, create it
    const from_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    // getting the token account of the toWallet address, and if it does not exist, create it
    const to_ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      to
    );

    // transfer the tokens to the 'to' token account we just created
    const tx = await transfer(
      connection,
      keypair,
      from_ata.address,
      to_ata.address,
      keypair,
      5_000_000 // 5 dosa coins
    );

    console.log(tx);
  } catch (e) {
    console.log("oops! something went wrong: ", e);
  }
})();
