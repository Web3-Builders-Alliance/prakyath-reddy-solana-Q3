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
    const { nft } = await metaplex.nfts().create({
      name: "my rug",
      uri: "https://arweave.net/Yez33kPt361DVhy3kN1BilaK6DaEs7dNks0LnP9bHB4",
      sellerFeeBasisPoints: 0,
      symbol: "RUG",
      isMutable: true,
      creators: [
        {
          address: keypair.publicKey,
          share: 100,
        },
      ],
    });

    console.log(
      `Success! Check out your NFT here:\nhttps://explorer.solana.com/address/${nft.address.toBase58()}?cluster=devnet`
    );
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
