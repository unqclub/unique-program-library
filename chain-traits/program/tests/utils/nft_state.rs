use std::collections::HashMap;

use chain_traits::{
    instruction::{CreateTraitConfigArgs, TraitAction, TraitValueAction},
    state::{AvailableTrait, TraitConfigKey},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct UriMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Trait>,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct Trait {
    pub trait_type: String,
    pub value: String,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct Property {
    category: Option<String>,
    files: Vec<File>,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct File {
    uri: String,
    ttype: String,
}

impl UriMetadata {
    pub fn get_traits() -> Vec<CreateTraitConfigArgs> {
        vec![
            CreateTraitConfigArgs {
                name: "Background".to_string(),
                values: vec![
                    TraitValueAction {
                        name: "Vanilla Ice".to_string(),
                        action: TraitAction::Add,
                    },
                    TraitValueAction {
                        name: "Ruby Red".to_string(),
                        action: TraitAction::Add,
                    },
                ],
            },
            CreateTraitConfigArgs {
                name: "Clothes".to_string(),
                values: vec![TraitValueAction {
                    name: "Nice Overalls".to_string(),
                    action: TraitAction::Add,
                }],
            },
            CreateTraitConfigArgs {
                name: "Eyewear".to_string(),
                values: vec![TraitValueAction {
                    name: "Nouns".to_string(),
                    action: TraitAction::Add,
                }],
            },
            CreateTraitConfigArgs {
                name: "Face".to_string(),
                values: vec![TraitValueAction {
                    action: TraitAction::Add,
                    name: "Smirk".to_string(),
                }],
            },
            CreateTraitConfigArgs {
                name: "Fur".to_string(),
                values: vec![TraitValueAction {
                    action: TraitAction::Add,
                    name: "Eggnog".to_string(),
                }],
            },
            CreateTraitConfigArgs {
                name: "Head".to_string(),
                values: vec![TraitValueAction {
                    action: TraitAction::Add,
                    name: "Spiky Hair".to_string(),
                }],
            },
            CreateTraitConfigArgs {
                name: "1/1".to_string(),
                values: vec![TraitValueAction {
                    action: TraitAction::Add,
                    name: "None".to_string(),
                }],
            },
        ]
    }

    pub fn map_traits() -> HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> {
        let traits = Self::get_traits();
        let mut trait_map: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> = HashMap::new();

        traits
            .iter()
            .enumerate()
            .for_each(|(name_index, trait_info)| {
                let mut trait_values: HashMap<u8, AvailableTrait> = HashMap::new();

                trait_info.values.iter().enumerate().for_each(|(index, v)| {
                    trait_values.insert(
                        index as u8,
                        AvailableTrait {
                            value: v.name.clone(),
                            is_active: true,
                        },
                    );
                });

                trait_map.insert(
                    TraitConfigKey {
                        name: trait_info.name.clone(),
                        id: name_index as u8,
                    },
                    trait_values,
                );
            });

        trait_map
    }
}
