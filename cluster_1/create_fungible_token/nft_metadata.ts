import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js";
import { Commitment, Connection, Keypair } from "@solana/web3.js";
import { readFile } from "fs/promises";

import * as bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

// declare our wallet secret key
const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;
const secretKeyUint8Array = bs58.decode(MY_WALLET_SECRET_KEY);
const keypair = Keypair.fromSecretKey(secretKeyUint8Array);

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const metaplex = Metaplex.make(connection)
  .use(keypairIdentity(keypair))
  .use(
    bundlrStorage({
      address: "https://devnet.bundlr.network",
      providerUrl: "https://api.devnet.solana.com",
      timeout: 60000,
    })
  );

(async () => {
  try {
    // read the image file
    const img = await readFile("./images/rug.png");
    const metaplexImg = toMetaplexFile(img, "1.png");
    const imgUri = await metaplex.storage().upload(metaplexImg);

    console.log(
      `You've uploaded your image:\n\n${imgUri}\n\nSave this URI so you can use it to mint an NFT!`
    );
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
