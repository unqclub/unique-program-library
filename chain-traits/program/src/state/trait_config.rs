use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]

pub struct TraitConfig {
    pub collection: Pubkey,
    pub update_authoirty: Pubkey,
    pub available_traits: HashMap<String, AvailableTrait>,
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct AvailableTrait {
    pub value: String,
    pub is_active: bool,
}
