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

**Challenge 3:**

- Load the IDL for the WBA Vault
- vault_init.ts - Initialize an Account with WBA Vault
- vault_deposit.ts - Deposit native Solana
- vault_withdraw.ts - Withdraw native Solana
- vault_deposit_spl.ts - Deposit your SPL token
- vault_withdraw_spl.ts - Withdraw SPL token
- Each successful interaction increases your WBA Score.

**Challenge 4:**

- nft image.ts

  - Generate and upload rug image to Bundlr
  - https://deanmlittle.github.io/generug/

- nft_metadata.ts

  - Using the metaplex JSON standard upload NFT Metadata to Bundlr
  - https://docs.metaplex.com/programs/token-metadata/token-standard#the-non-fungible-standard

- nft_mint.ts
  - use metaplex.nfts().create to mint an NFT using your uploaded metadata

**Challenge 5:**

- Create a new project by running anchor init wba_vault
- We are going to add 3 instructions,
  - Initialize
  - Deposit
  - Withdraw
- Use wba_init.ts, wba_deposit.ts & wba_withdraw.ts to test the contract
- Run anchor test to build, deploy to localnet and run tests located in the test folder.
