// import connection to solana devnet
import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

// import wallet from ./dev-wallet.json
import wallet from "./dev-wallet.json";

// import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// create a devnet connection to sirdrop sol tokens
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// claim 2 devnet sol tokens
(async () => {
  try {
    // we are going to claim 2 devnet sol tokens
    const txhash = await connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    console.log(
      `Success! check your txhash: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (e) {
    console.error("oops, something went wrong", e);
  }
})();
