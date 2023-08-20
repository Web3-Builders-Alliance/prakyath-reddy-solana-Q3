// use findProgramAddressSync to get the PDA for the metadata of your mint
import {
  Connection,
  Keypair,
  Transaction,
  PublicKey,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
//import wallet from "../../wba-wallet.json";
import {
  createCreateMetadataAccountV2Instruction,
  createCreateMetadataAccountV3Instruction,
} from "@metaplex-foundation/mpl-token-metadata";
import * as bs58 from "bs58";
import dotenv from "dotenv";
dotenv.config();

// importing keypair from wallet file
//const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const MY_WALLET_SECRET_KEY = process.env.MY_WALLET_SECRET_KEY!;
const secretKeyUint8Array = bs58.decode(MY_WALLET_SECRET_KEY);
const keypair = Keypair.fromSecretKey(secretKeyUint8Array);

// create a solana devnet connection
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// define our mint address
const mintAddress = new PublicKey(
  "4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN"
);

// add the token metadata program
const tokenMetadataProgramId = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// create PDA for token metadata
const metadataSeeds = [
  Buffer.from("metadata"),
  tokenMetadataProgramId.toBuffer(),
  mintAddress.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
  metadataSeeds,
  tokenMetadataProgramId
);

(async () => {
  try {
    // create a transaction
    const tx = new Transaction().add(
      createCreateMetadataAccountV3Instruction(
        {
          metadata: metadata_pda,
          mint: mintAddress,
          mintAuthority: keypair.publicKey,
          payer: keypair.publicKey,
          updateAuthority: keypair.publicKey,
        },
        {
          createMetadataAccountArgsV3: {
            data: {
              name: "dosa coin",
              symbol: "DOSA",
              uri: "",
              sellerFeeBasisPoints: 0,
              creators: null,
              collection: null,
              uses: null,
            },
            isMutable: true,
            collectionDetails: null,
          },
        }
      )
    );
    const txhash = await sendAndConfirmTransaction(connection, tx, [keypair]);
  } catch (e) {
    console.error("something went wrong: ", e);
  }
})();
