# Fitting the Stacks chain into an in-memory datastore

It's relatively easy, actually. The memory store generally is larger than the chain itself as it has to store the raw data + parsed data + references between keys.

We have four types of data in the datastore:

- **Principals** (regular principals + smart contracts)
- **Blocks** (anchor blocks + microblocks)
- **Transactions**
- **Traits** (specifications for smart contracts)

Stacks uses an account model, which is pretty useful when using a key-value store.

Below is a list of all the different types of keys that we use in the datastore.

| **Key**                                                               | **Redis data type** | **Description**                                        |
| --------------------------------------------------------------------- | ------------------- | ------------------------------------------------------ |
| `tx:<TXID>`                                                           | `HASH`              | a transaction (parsed + raw)                           |
| `block:<BLOCKHASH>`                                                   | `HASH`              | a block (includes txids + other block data)            |
| `block:txs:<BLOCKHASH>`                                               | `LIST`              | a list of txids in a block                             |
| `block:height:<BLOCKHEIGHT>`                                          | `STRING`            | the block hash of the block at a given height          |
| `principal:txs:<PRINCIPAL>`                                           | `LIST`              | a list of transaction IDs associated with a principal  |
| `principal:balance:<PRINCIPAL>`                                       | `HASH`              | the principal's balance of NFTs, FTs and STX           |
| `contract:source:<PRINCIPAL>`                                         | `STRING`            | the source code of a smart contract                    |
| `contract:txid:<PRINCIPAL>`                                           | `STRING`            | the transaction ID of the smart contract's deployment  |
| `contract:abi:<PRINCIPAL>`                                            | `HASH`              | the ABI (interface) of the contract                    |
| `trait:contracts:<TRAIT_DEPLOYER>.<TRAIT_CONTRACT_NAME>.<TRAIT_NAME>` | `LIST`              | a list of contracts IDs that implement a certain trait |

Details on these keys below.

## Transactions

The majority of the data on a blockchain are transactions, which fits well into a KV store as each transaction can have a key (the transaction hash / TXID) and a value (the transaction itself).

- `tx:<TXID>` | `HASH` | a transaction

## Blocks

Another use for data are the blocks that contain the transactions. Blocks are stored as a key-value pair, where the key is the block hash and the value is the block itself. 

- `block:<BLOCKHASH>` | `HASH` | a block (includes raw tx + other block data)
- `block:txs:<BLOCKHASH>` | `LIST` | a list of txids in a block
- `block:height:<BLOCKHEIGHT>` | `STRING` | the block hash of the block at a given height

## Principals

There are two types of principals: smart contracts and accounts.

We want to index the accounts' transactions (we do this through a list of txids to the transactions) and we also store the accounts' balances of NFTs, FTs and STX. In the latter, we also store the total inbound and outbound assets.

A `PRINCIPAL` is `<ADDRESS>` for an account or `<DEPLOYER_ADDRESS>.<CONTRACT_NAME>` for a smart contract.

- `principal:txs:<PRINCIPAL>` | `LIST` | a list of transaction IDs associated with a principal
- `principal:balance:<PRINCIPAL>` | `HASH` | the principal's balance of NFTs, FTs and STX
- `contract:source:<PRINCIPAL>` | `STRING` | the source code of a smart contract
- `contract:txid:<PRINCIPAL>` | `STRING` | the transaction ID of the smart contract's deployment
- `contract:abi:<PRINCIPAL>` | `HASH` | the ABI (interface) of the contract

## Traits

Traits are specifications for smart contracts. They include a list of functions that the contract contains, as well as their response types. They're commonly used to implement things like NFT & FT standards.

- `trait:contracts:<TRAIT_DEPLOYER>.<TRAIT_CONTRACT_NAME>.<TRAIT_NAME>` | `LIST` | a list of contracts IDs that implement a certain trait
