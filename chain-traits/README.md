# Unique Chain Traits - UCT

Unique Chain Traits is a simple tool to manage NFT collection’s on-chain traits on Solana.

# About

With UCT, you can transfer the entire data for your collection’s traits onto chain, as well as manage them afterwards - change certain NFT’s traits, remove certain traits, and more.
Having traits on-chain allows programs to directly interact with them, which opens up a wide range of use cases: gating access with traits, trading certain traits of the collections fully on chain - and those are just about the collections, however, for example, for ticketing systems and other applications of NFTs even more cases can arise.
UCT is fully open source, and while we did build a front-end for it as well, we encourage everyone to contribute, integrate, and build with UCT.

# Background

Previously, most collections only had there trait data off-chain, which is suboptimal not just from the perspective of decentralization (since that data is stored on a server) but also from the user experience perspective, since on-chain programs don’t have direct access to off-chain information.

# Documentation

## Program State

The UCT program utilizes two Program Derived Accounts:

- Trait Config
- Trait Data

### The Trait Config

Trait config account is used to store all available tratis metadata. Only update authoirty of certain collection is authorized to modify data on account related with his collection. (On Devnet, this constraint is disabled due to testing purposes)

```rust
pub struct TraitConfig {
    //Collection key (First creator if NFTs do not have collection)
    pub collection: Pubkey,
    // Pubkey of collection update authority (From metaplex metadata)
    pub update_authoirty: Pubkey,
    // Unix timestamp of last account modification
    pub last_modified: i64,
    //All available traits for certain collection
    pub available_traits: HashMap<String, Vec<AvailableTrait>>,
}
```

The `AvailableTrait` struct is used to store data about each trait value.

```rust
pub struct AvailableTrait {
    //String representing trait value
    pub value: String,
    //Flag indicating if value is still available (used to enable update authority to disable traits values)
    pub is_active: bool,
}
```

### The Trait Data

Trait Data is account that is related with each NFT from collection and is used to store traits for single NFT on chain.
For already existing collections, only `Update Authority` of collection can store and modify data on this account. In case of minting new NFT, `User` who mints is authorized to create this account for his new NFT and store traits.

```rust
pub struct TraitData {
    //Related trait config PDA address
    pub trait_config: Pubkey,
    // Mint of nft to which TraitData has 1-1 relation
    pub nft_mint: Pubkey,
    // Unix timestamp of last modification
    pub last_modified: i64,
    // Map storing all trait-value combinations
    pub traits: HashMap<String, String>,
}
```

When storing or modifying data on `TraitData` account, `TraitConfig` must be passed in instruction,and every `TraitValue` that you want to store on account, has to exist in TraitConfig's `available_traits` HashMap.

# Devnet address

The version of the program compiled from the source code found in this repository is deployed on devnet @ `EVYWVskBg3xFwL3Px5FvGY3iC3kR4n4Mo9AA8kpTk1JB`

# License

Unique Chain Traits is licensed under the GNU Affero General Public License v3.0.

In short, this means that any changes to this code must be made open source and available under the AGPL-v3.0 license, even if only used privately.
