use std::collections::HashMap;

use chain_traits::{
    instruction::CreateTraitConfigArgs,
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
                action: chain_traits::instruction::TraitAction::Add,
                name: "Background".to_string(),
                values: vec!["Vanilla Ice".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Clothes".to_string(),
                values: vec!["Nice Overalls".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Eyewear".to_string(),
                values: vec!["Nouns".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Face".to_string(),
                values: vec!["Smirk".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Fur".to_string(),
                values: vec!["Eggnog".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Head".to_string(),
                values: vec!["Spiky Hair".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "1/1".to_string(),
                values: vec!["None".to_string()],
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
                            value: v.clone(),
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
