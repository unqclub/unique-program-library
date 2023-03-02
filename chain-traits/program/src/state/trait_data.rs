use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::id;

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TraitData {
    pub nft_mint: Pubkey,
    pub last_modified: i64,
    pub traits: HashMap<String, String>,
}
impl TraitData {
    pub const LEN: usize = 32 + 8;

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
