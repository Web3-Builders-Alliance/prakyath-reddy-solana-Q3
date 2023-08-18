Creating a Fungible Token

- spl_init.ts
- Create a new token mint
- Make your WBA devnet wallet the mint authority
- Set Decimals to 6 o console log the mint id

spl_mint.ts

- use getOrCreateAssociatedTokenAccount to create a token account using your wallet and the mint id you logged...
- use mintTo to mint token to your self
