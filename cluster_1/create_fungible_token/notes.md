cargo install spl-token-cli  
yarn add @solana/spl-token  
source: https://spl.solana.com/token#creating-a-token-mint  
ts-node ./mnemonic_to_uint8array.ts  
mint id: 4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN  
spl-token supply 4RKW1u4XXgQJd3PB8VYmHTarkmL1GT58TDSSak76uVnN - shows supply of our token  
Your ATA is: 3zvdsJY9YbfyYNA69kAR4NDDvXqgronMge8aPLjk8UnT  
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

vault_deposit: https://explorer.solana.com/tx/5opmto2wVdcgenM59GK5mQ4JV2uM8aGRyfQRkUa5yhrAgM8cNvq3wrmsaBZkujEfXdRFpV38D42Qcre4zX6GA3hr?cluster=devnet

rug image uploaded to arveave when nft_image program is run - https://arweave.net/qVGj2SKyPDuayi8g-HX2IO2N8aCDGNQLUFqUdbMvtig

If u try to put the arweave link directly into metaplex as the URI, instead of doing the metadata JSON file, it won't show up

Find the Non-Fungible Standard JSON example in this link - https://docs.metaplex.com/programs/token-metadata/token-standard

metadata URI from nft_metadata program - https://arweave.net/Yez33kPt361DVhy3kN1BilaK6DaEs7dNks0LnP9bHB4

Success! Check out your NFT here:
https://explorer.solana.com/address/7XKj4rnUgKCkTadBGfRVb4EhrVz3NmkL1Y7XnzDnNokU?cluster=devnet