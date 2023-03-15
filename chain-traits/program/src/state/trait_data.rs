use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::id;

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
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
impl TraitData {
    pub const LEN: usize = 32 + 32 + 8;

    pub fn get_trait_data_seeds<'a>(
        nft_mint: &'a Pubkey,
        trait_config: &'a Pubkey,
    ) -> [&'a [u8]; 3] {
        [b"trait-data", nft_mint.as_ref(), trait_config.as_ref()]
    }
}

pub fn find_trait_data_address(trait_config: &Pubkey, nft_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"trait-data", nft_mint.as_ref(), trait_config.as_ref()],
        &id(),
    )
}
