**Challenge 1:**
Creating a Fungible Token

spl_init.ts

- Create a new token mint
- Make your WBA devnet wallet the mint authority
- Set Decimals to 6 to console log the mint id

spl_mint.ts

- use getOrCreateAssociatedTokenAccount to create a token account using your wallet and the mint id you logged...
- use mintTo to mint token to your self

**Challenge 2:**

spl_metadata.ts

- Use findProgramAddressSync to get the PDA for the Metadata for your mint.
- Create a new Transaction
- add a createCreateMetadataAccountV3Instruction to it
- add the required Accounts and Data
- use sendAndConfirm Transaction to send it to Devnet

spl_transfer.ts

- Transfer tokens to another cadet.
- use getOrCreateAssociatedTokenAccount to get the token accounts