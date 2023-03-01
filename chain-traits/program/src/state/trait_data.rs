use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TraitData {
    pub nft_mint: Pubkey,
    pub last_modified: i64,
    pub traits: HashMap<String, String>,
}
impl TraitData {
    pub const LEN: usize = 32 + 8;
}
