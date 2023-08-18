// create a new token mint
// https://spl.solana.com/token#creating-a-token-mint

import { createMint } from "@solana/spl-token";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";

const payer = Keypair.generate();
const mintAuthority = Keypair.generate();
const freezeAuthority = Keypair.generate();
