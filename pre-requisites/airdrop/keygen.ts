// here, we are going to generate ourselves a new keypair

// import keypair from @solana/web3.js
import { Keypair } from "@solana/web3.js";

// generate a new keypair
let kp = Keypair.generate();
console.log(
  `you've generated a new solana wallet! - ${kp.publicKey.toBase58()}`
);
console.log(`secret key: ${kp.secretKey}`);
