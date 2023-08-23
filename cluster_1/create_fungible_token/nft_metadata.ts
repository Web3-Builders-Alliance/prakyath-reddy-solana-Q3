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
    const { uri: metadataUri } = await metaplex.nfts().uploadMetadata({
      name: "my rug",
      description: "We're not the same bro",
      image: "https://arweave.net/qVGj2SKyPDuayi8g-HX2IO2N8aCDGNQLUFqUdbMvtig",
      attributes: [
        {
          trait_type: "background",
          value: "blue",
        },
        {
          trait_type: "pattern",
          value: "dots",
        },
      ],
      properties: {
        files: [
          {
            uri: "https://arweave.net/qVGj2SKyPDuayi8g-HX2IO2N8aCDGNQLUFqUdbMvtig",
            type: "image/png",
          },
        ],
        category: "image",
      },
    });

    console.log(
      `You've uploaded your image:\n\n${metadataUri}\n\nSave this URI so you can use it to mint an NFT!`
    );
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
