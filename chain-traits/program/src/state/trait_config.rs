use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::{
    id,
    instruction::{CreateTraitConfigArgs, TraitAction},
};

#[derive(ShankAccount, BorshDeserialize, BorshSerialize, Clone, Debug)]

pub struct TraitConfig {
    pub collection: Pubkey,
    pub update_authoirty: Pubkey,
    pub last_modified: i64,
    pub available_traits: HashMap<String, Vec<AvailableTrait>>,
}

impl TraitConfig {
    pub const LEN: usize = 32 + 32 + 8;

    pub fn traits_to_map(
        traits: Vec<CreateTraitConfigArgs>,
    ) -> HashMap<String, Vec<AvailableTrait>> {
        let mut trait_map: HashMap<String, Vec<AvailableTrait>> = HashMap::new();
        traits.iter().for_each(|trait_info| {
            trait_map.insert(
                trait_info.name.clone(),
                trait_info
                    .values
                    .iter()
                    .map(|traits_data| AvailableTrait {
                        value: traits_data.clone(),
                        is_active: trait_info.action == TraitAction::Add,
                    })
                    .collect(),
            );
        });

        trait_map
    }

    pub fn map_available_traits(traits: Vec<String>, is_active: bool) -> Vec<AvailableTrait> {
        traits
            .iter()
            .map(|trait_info| AvailableTrait {
                is_active,
                value: trait_info.clone(),
            })
            .collect()
    }

    pub fn update_traits(&mut self, new_values: &Vec<String>, action: &TraitAction, name: &String) {
        if let Some(existing_trait) = self.available_traits.get_mut(name) {
            for new_val in new_values.iter() {
                if let Some(existing_value) = existing_trait
                    .iter_mut()
                    .find(|v| v.value == new_val.clone())
                {
                    existing_value.is_active = TraitAction::Add == *action;
                } else {
                    existing_trait.push(AvailableTrait {
                        value: new_val.clone(),
                        is_active: *action == TraitAction::Add,
                    });
                }
            }
        } else {
            self.available_traits.insert(
                name.clone(),
                new_values
                    .iter()
                    .map(|val| AvailableTrait {
                        is_active: *action == TraitAction::Add,
                        value: val.clone(),
                    })
                    .collect(),
            );
        }
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
