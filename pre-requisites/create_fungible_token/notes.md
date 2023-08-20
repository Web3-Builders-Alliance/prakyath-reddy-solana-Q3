cargo install spl-token-cli  
yarn add @solana/spl-token  
source: https://spl.solana.com/token#creating-a-token-mint  
ts-node ./mnemonic_to_uint8array.ts  
mint id: 4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN  
spl-token supply 4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN - shows supply of our token  
Your ata is: 3zvdsJY9YbfyYNA69kAR4NDDvXqgronMge8aPLjk8UnT  
Your mint txn id is: 3zvdsJY9YbfyYNA69kAR4NDDvXqgronMge8aPLjk8UnT  
spl-token supply 4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN: 10  
JSON token standards - https://docs.metaplex.com/programs/token-metadata/token-standard
metadata program id: metagbxxUerda28ci1RbAWKYOm3vbzib6a8bt518×1s
metadata typescript library: @metaplex-foundation/mpl-token-metadata

for challenge 2, we are not actually creating an NFT, we're going to be creating a fungible token that has metadata associated with it.
Docs for createMetadataAccountV2Instructions - https://docs.metaplex.com/programs/token-metadata/instructions#create-a-metadata-account

Jeff's solana address: BvhV49WPYBbzPu8Fpy8YnPnwhNWLbm9Vmdj2T5bNSotS
My solana address: 7sCXq3U28U2R9XNtnzfXCXUhLidRdfsgwt6gtFytRL33

IDL Vault: https://explorer.solana.com/address/D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o?cluster=devnet

when u create wba_vault.ts, anchor should be able to load that IDL and you'll be able to use the methods from the typescript library.

vault_init tx hash: https://explorer.solana.com/tx/57htmra1k9L9FzfZdpNeHSZtQkgADDzUgT7zpdk3wi1o9N8j7kTAtw3E8x433rkayPrxpZ57kcX7unbRU3AyyC3k?cluster=devnet

vault state: 8oJB65i8gDMwMWTgk2mozdkeF2bMFQLSv6LcG4npAU5A - if you go to Anchor Data on solscan for this address, it shows the state data for your vault