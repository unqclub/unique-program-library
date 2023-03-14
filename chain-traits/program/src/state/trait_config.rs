use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::{
    id,
    instruction::{CreateTraitConfigArgs, TraitAction, TraitValueAction},
};

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TraitConfig {
    pub collection: Pubkey,
    pub update_authority: Pubkey,
    pub last_modified: i64,
    pub available_traits: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct TraitConfigKey {
    pub name: String,
    pub id: u8,
}

impl TraitConfig {
    pub const LEN: usize = 32 + 32 + 8;

    pub fn traits_to_map(
        traits: &Vec<CreateTraitConfigArgs>,
    ) -> HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> {
        let mut trait_map: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> = HashMap::new();
        traits
            .iter()
            .enumerate()
            .for_each(|(name_index, trait_info)| {
                trait_map.insert(
                    TraitConfigKey {
                        id: name_index as u8,
                        name: *trait_info.name.clone(),
                    },
                    *trait_info.values.clone(),
                );
            });

        trait_map
    }

    pub fn map_available_traits(traits: &Vec<TraitValueAction>) -> HashMap<u8, AvailableTrait> {
        let mut available_traits: HashMap<u8, AvailableTrait> = HashMap::new();

        traits.iter().enumerate().for_each(|(index, value)| {
            available_traits.insert(
                index.try_into().unwrap(),
                AvailableTrait {
                    value: value.name.clone(),
                    is_active: value.action == TraitAction::Add,
                },
            );
        });

        available_traits
    }
    pub fn get_trait_config_seeds<'a>(collection: &'a Pubkey) -> [&'a [u8]; 2] {
        [b"trait-config", collection.as_ref()]
    }
}

pub fn find_trait_config_address(collection: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"trait-config", collection.as_ref()], &id())
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct AvailableTrait {
    pub value: String,
    pub is_active: bool,
}
