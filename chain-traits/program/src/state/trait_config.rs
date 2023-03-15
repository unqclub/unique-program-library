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
    //Collection key (First creator if NFTs do not have collection)
    pub collection: Pubkey,
    // Pubkey of collection update authority (From metaplex metadata)
    pub update_authoirty: Pubkey,
    // Unix timestamp of last account modification
    pub last_modified: i64,
    //All available traits for certain collection
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
                        value: traits_data.to_string(),
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
                value: trait_info.to_string(),
            })
            .collect()
    }

    pub fn update_traits(&mut self, new_values: &Vec<String>, action: &TraitAction, name: &String) {
        if let Some(existing_trait) = self.available_traits.get_mut(name) {
            if action == &TraitAction::Add {
                let mapped_args: &mut Vec<AvailableTrait> = &mut new_values
                    .iter()
                    .map(|val| AvailableTrait {
                        is_active: true,
                        value: val.clone(),
                    })
                    .collect();
                existing_trait.append(mapped_args);
            } else {
                for new_val in new_values.iter() {
                    let existing_value = existing_trait
                        .iter_mut()
                        .find(|v| &v.value == new_val)
                        .unwrap();

                    existing_value.is_active = TraitAction::Modify == *action;
                }
            }
        } else {
            self.available_traits.insert(
                name.clone(),
                new_values
                    .iter()
                    .map(|val| AvailableTrait {
                        is_active: *action == TraitAction::Add,
                        value: val.to_string(),
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
