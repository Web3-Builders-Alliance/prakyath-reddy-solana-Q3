// import transaciton, systemprogram, sendand confirm trasaction, connection, keypair, lamports per sol, public key from @solana/web3.js
import {
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";

// import dev wallet from ./dev-wallet.json
import wallet from "./dev-wallet.json";

// import our dev-wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// define our WBA public key
const to = new PublicKey("7sCXq3U28U2R9XNtnzfXCXUhLidRdfsgwt6gtFytRL33");

// create a solana devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// use solana/web3.js to transfer 0.1 sol from 'from' to 'to' wallet
(async () => {
  try {
    // display initial balance in 'from' wallet
    const balance = await connection.getBalance(from.publicKey);
    console.log(`Your wallet balance is ${balance / LAMPORTS_PER_SOL} SOL`);

    // create a test transaction to canlculate fees
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100,
      })
    );
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;
    transaction.feePayer = from.publicKey;

    // calculate the cost of sending the transaction
    const fee =
      (
        await connection.getFeeForMessage(
          transaction.compileMessage(),
          "confirmed"
        )
      ).value || 0;

    // remove our transfer instruction and replace it with a transfer of the remaining balance
    transaction.instructions.pop();

    // now add the instruction with the correct lamports amount
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    // sign the transaction, broadcast it, and confirm it
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);

    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );

    // display final balance in 'from' wallet
    const finalBalance = await connection.getBalance(from.publicKey);
    console.log(
      `Your final wallet balance is ${finalBalance / LAMPORTS_PER_SOL} SOL`
    );
    // display final balance in 'to' wallet
    const finalBalanceTo = await connection.getBalance(to);
    console.log(
      `Your final wallet balance is ${finalBalanceTo / LAMPORTS_PER_SOL} SOL`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
