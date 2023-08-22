// use getOrCreateAssociatedTokenAccount to create a token account using your wallet and the mint id you logged
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
// import wallet from "../wba-wallet.json";
import * as bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;
const secretKeyUint8Array = bs58.decode(MY_WALLET_SECRET_KEY);
const walletKeypair = Keypair.fromSecretKey(secretKeyUint8Array);
const mintAuthority = walletKeypair; // Using my WBA wallet as the mint authority

// import our keypair from the wallet file
//const keypair = Keypair.fromSecretKey(new Uint8Array());

// create a solana devnet connection
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const token_decimals = 1_000_000; // 6 decimals

// mint address
const mint = new PublicKey("4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN");

// create a token account for the mint
(async () => {
  try {
    // create an ATA for the mint
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      walletKeypair,
      mint,
      walletKeypair.publicKey
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);

    // mint 10 tokens to the ATA
    await mintTo(
      connection,
      walletKeypair,
      mint,
      ata.address,
      mintAuthority,
      10000000 // because decimals for the mint are set to 6
    );
    console.log(`Your mint txn id is: ${ata.address.toBase58()}`);
  } catch (error) {
    console.log("oops! something went wrong: ", error);
  }
})();
