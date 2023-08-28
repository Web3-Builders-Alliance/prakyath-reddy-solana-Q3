import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, WbaVault } from "../target/types/wba_vault";
import { PublicKey, Keypair, Commitment, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

describe("wba_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const owner = new Keypair();
  const commitment: Commitment = "confirmed";

  const programId = new PublicKey(
    "wbauEhzu1CGBTsbzW2VpFfKrsStuNDi7YMw3Uj5WBvf"
  );
  const program = new anchor.Program<WbaVault>(
    IDL,
    programId,
    anchor.getProvider()
  ) as Program<WbaVault>;

  const state = PublicKey.findProgramAddressSync(
    [Buffer.from("state"), program.programId.toBytes()],
    program.programId
  )[0];
  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), program.programId.toBytes()],
    program.programId
  )[0];
  const auth = PublicKey.findProgramAddressSync(
    [Buffer.from("auth"), program.programId.toBytes()],
    program.programId
  )[0];

  it("Airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        owner.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("Initialize", async () => {
    await program.methods
      .initialize()
      .accounts({
        owner: owner.publicKey,
        auth,
        vault,
        state,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        owner.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "confirmed"
  );
  return signature;
};
